use std::{
    env, fs,
    io::Write,
    path::{Path, PathBuf},
    sync::OnceLock,
};

pub mod common;

static AOC_URL: &str = "https://adventofcode.com";

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

fn download_file(url: &str) -> String {
    let response = ureq::get(url).set("Cookie", get_session()).call().unwrap();

    if response.status() != 200 {
        panic!(
            "Response status code {}: make sure AOC_SESSION is set to a valid session",
            response.status()
        );
    }

    response.into_string().unwrap()
}

pub fn load_input(year: u16, day: u8) -> String {
    let file_path = get_cache().join(format!("input-{year}-{day:02}.txt"));

    if !Path::new(&file_path).is_file() {
        let url = format!("{AOC_URL}/{year}/day/{day}/input");
        let input = download_file(&url);
        let mut file = fs::File::create(&file_path).unwrap();
        file.write_all(input.as_bytes()).unwrap();
        return input;
    }

    fs::read_to_string(&file_path).unwrap()
}

pub fn auto_input(path: &str) -> String {
    let year = path.split_once('/').unwrap().0.parse::<u16>().unwrap();

    let day = path
        .rsplit_once("day")
        .unwrap()
        .1
        .strip_suffix(".rs")
        .unwrap()
        .parse::<u8>()
        .unwrap();

    load_input(year, day)
}

#[macro_export]
macro_rules! solution {
    () => {
        $crate::solution!(2, true, true);
    };
    ($run_part1:expr, $run_part2:expr) => {
        $crate::solution!(2, $run_part1, $run_part2);
    };
    (1) => {
        use $crate::common::*;

        fn main() {
            let input = $crate::auto_input(file!());
            let answer = part1(&input);
            println!("Part 1 answer: {answer}");
        }
    };
    (2, $run_part1:expr, $run_part2:expr) => {
        use $crate::common::*;

        fn main() {
            let input = $crate::auto_input(file!());

            if $run_part1 {
                let answer = part1(&input);
                println!("Part 1 answer: {answer}");
            }

            if $run_part2 {
                let answer = part2(&input);
                println!("Part 2 answer: {answer}");
            }
        }
    };
}
