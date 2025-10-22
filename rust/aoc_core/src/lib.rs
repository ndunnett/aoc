use std::{
    env, fs,
    io::{Read, Write},
    path::{Path, PathBuf},
    sync::OnceLock,
};

const MAX_REQUESTS: usize = 5; // 5 requests per request period
const REQUEST_PERIOD: u64 = 15 * 60; // request period of 15 minutes
const USER_AGENT: &str = "github.com/ndunnett/aoc/rust";
const LOG_FILE: &str = "log.txt";

pub const AOC_URL: &str = "https://adventofcode.com";

fn get_cache() -> &'static PathBuf {
    static CACHE_PATH: OnceLock<PathBuf> = OnceLock::new();

    CACHE_PATH.get_or_init(|| {
        let path = env::current_exe()
            .unwrap()
            .parent()
            .unwrap()
            .join("../../../.cache");

        if !path.exists() {
            fs::create_dir(&path).unwrap();
        }

        path
    })
}

fn get_session() -> &'static String {
    static SESSION: OnceLock<String> = OnceLock::new();

    SESSION.get_or_init(|| format!("session={}", env::var("AOC_SESSION").unwrap()))
}

fn throttle_requests() -> anyhow::Result<()> {
    let path = get_cache().join(LOG_FILE);
    let path = Path::new(&path);

    if path.is_file() {
        let mut timestamps = {
            let mut file = fs::File::open(path)?;
            let mut content = String::new();
            file.read_to_string(&mut content)?;

            content
                .lines()
                .map(chrono::DateTime::parse_from_rfc3339)
                .collect::<Result<Vec<_>, chrono::ParseError>>()?
        };

        if timestamps.len() >= MAX_REQUESTS {
            let diff = chrono::Local::now()
                .signed_duration_since(timestamps[0])
                .num_seconds()
                .unsigned_abs();

            if diff < REQUEST_PERIOD {
                let duration = std::time::Duration::from_secs(REQUEST_PERIOD - diff);
                println!("Request throttled, waiting {duration:#?} before downloading input file.");
                std::thread::sleep(duration);
            }
        }

        timestamps.push(chrono::Local::now().fixed_offset());
        let range = timestamps.len().saturating_sub(MAX_REQUESTS)..timestamps.len();

        let content = timestamps[range]
            .iter()
            .map(chrono::DateTime::to_rfc3339)
            .collect::<Vec<_>>()
            .join("\n");

        let mut file = fs::File::create(path)?;
        file.write_all(content.as_bytes())?;
    } else {
        let mut file = fs::File::create(path)?;
        let content = chrono::Local::now().to_rfc3339();
        file.write_all(content.as_bytes())?;
    }

    Ok(())
}

fn download_file(url: &str) -> anyhow::Result<String> {
    throttle_requests()?;

    let response = ureq::get(url)
        .header("Cookie", get_session())
        .header("User-Agent", USER_AGENT)
        .call()?;

    if response.status() != 200 {
        Err(anyhow::anyhow!(
            "Response status code {}: make sure AOC_SESSION is set to a valid session",
            response.status()
        ))
    } else {
        Ok(response.into_body().read_to_string()?)
    }
}

pub fn delete_cached_input(year: u16, day: u8) -> anyhow::Result<()> {
    let path = get_cache().join(format!("input-{year}-{day:02}.txt"));
    let path = Path::new(&path);

    if path.is_file() {
        fs::remove_file(path)?;
    }

    Ok(())
}

pub fn load_input(year: u16, day: u8) -> anyhow::Result<String> {
    let path = get_cache().join(format!("input-{year}-{day:02}.txt"));
    let path = Path::new(&path);

    if !path.is_file() {
        let url = format!("{}/{year}/day/{day}/input", crate::AOC_URL);
        let input = download_file(&url)?;
        let mut file = fs::File::create(path)?;
        file.write_all(input.as_bytes())?;
        Ok(input)
    } else {
        Ok(fs::read_to_string(path)?)
    }
}

pub fn auto_input(path: &str) -> anyhow::Result<String> {
    let year = path.split_once('/').unwrap().0.parse::<u16>()?;

    let day = path
        .rsplit_once("day")
        .unwrap()
        .1
        .strip_suffix(".rs")
        .unwrap()
        .parse::<u8>()?;

    load_input(year, day)
}
