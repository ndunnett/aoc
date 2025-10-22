use std::{env, fs, path::PathBuf, sync::OnceLock, time::Duration};

use anyhow::anyhow;
use chrono::{DateTime, Utc};

type Anyhow<T> = anyhow::Result<T>;

const MAX_REQUESTS: usize = 5; // 5 requests per request period
const REQUEST_PERIOD: u64 = Duration::from_mins(15).as_secs(); // request period duration
const USER_AGENT: &str = "github.com/ndunnett/aoc/rust";
const LOG_FILE: &str = "log.txt";

pub const AOC_URL: &str = "https://adventofcode.com";

fn get_cache() -> &'static PathBuf {
    static CACHE_PATH: OnceLock<PathBuf> = OnceLock::new();

    CACHE_PATH.get_or_init(|| {
        let path = env::current_exe()
            .expect("failed to get the current executable path")
            .parent()
            .expect("failed to get the current executable parent path")
            .join("../../../.cache");

        if let Err(e) = fs::create_dir_all(&path) {
            panic!("failed to create cache directory {path:?}: {e}");
        }

        path
    })
}

fn get_session() -> &'static String {
    static SESSION: OnceLock<String> = OnceLock::new();

    SESSION.get_or_init(|| {
        format!(
            "session={}",
            env::var("AOC_SESSION").expect("failed to get AOC_SESSION environment variable")
        )
    })
}

fn throttle_requests() -> Anyhow<()> {
    let path = get_cache().join(LOG_FILE);

    if path.is_file() {
        let timestamps = {
            fs::read_to_string(&path)?
                .lines()
                .map(|line| DateTime::parse_from_rfc3339(line).map(|dt| dt.with_timezone(&Utc)))
                .collect::<Result<Vec<_>, chrono::ParseError>>()
        }?;

        let now = Utc::now();

        if timestamps.len() >= MAX_REQUESTS {
            let elapsed = now
                .signed_duration_since(timestamps[0])
                .num_seconds()
                .max(0) as u64;

            if elapsed < REQUEST_PERIOD {
                let duration = Duration::from_secs(REQUEST_PERIOD - elapsed);
                println!("Request throttled, waiting {duration:#?} before downloading input file.");
                std::thread::sleep(duration);
            }
        }

        let now = Utc::now();

        let content = timestamps
            .into_iter()
            .chain([now])
            .filter_map(|t| {
                if (now.signed_duration_since(t).num_seconds().max(0) as u64) < REQUEST_PERIOD {
                    Some(t.to_rfc3339())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
            .join("\n");

        fs::write(path, content)?;
    } else {
        fs::write(path, Utc::now().to_rfc3339())?;
    }

    Ok(())
}

fn download_file(url: &str) -> Anyhow<String> {
    throttle_requests()?;

    let response = ureq::get(url)
        .header("Cookie", get_session())
        .header("User-Agent", USER_AGENT)
        .call()?;

    if !response.status().is_success() {
        Err(anyhow::anyhow!(
            "Response status code {}: make sure AOC_SESSION is set to a valid session",
            response.status()
        ))
    } else {
        Ok(response.into_body().read_to_string()?)
    }
}

pub fn delete_cached_input(year: u16, day: u8) -> Anyhow<()> {
    let path = get_cache().join(format!("input-{year}-{day:02}.txt"));

    if path.is_file() {
        fs::remove_file(path)?;
    }

    Ok(())
}

pub fn load_input(year: u16, day: u8) -> Anyhow<String> {
    let path = get_cache().join(format!("input-{year}-{day:02}.txt"));

    if !path.is_file() {
        let url = format!("{}/{year}/day/{day}/input", crate::AOC_URL);
        let input = download_file(&url)?;
        fs::write(path, &input)?;
        Ok(input)
    } else {
        Ok(fs::read_to_string(path)?)
    }
}

pub fn auto_input(path: &str) -> Anyhow<String> {
    let path = PathBuf::from(path);

    let day = path
        .file_stem()
        .map(|p| p.to_string_lossy())
        .ok_or_else(|| anyhow!("invalid file name"))?
        .trim_start_matches("day")
        .parse::<u8>()?;

    let year = path
        .parent()
        .and_then(|p| p.parent())
        .and_then(|p| p.parent())
        .and_then(|p| p.file_name())
        .ok_or_else(|| anyhow!("failed to extract year directory"))?
        .to_string_lossy()
        .parse::<u16>()?;

    load_input(year, day)
}
