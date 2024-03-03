extern crate lazy_static;
extern crate reqwest;

use std::env;
use std::fs;
use std::io;
use std::path;

fn download_file(url: String) -> String {
    let client = reqwest::blocking::Client::new();
    let session = env::var("AOC_SESSION").unwrap();
    client
        .get(url)
        .header("Cookie", format!("session={session}"))
        .send()
        .unwrap()
        .text()
        .unwrap()
}

pub fn load_input(day: i32) -> String {
    let cache_folder = "target/.cache";
    let file_path = format!("{cache_folder}/input-day-{day:02}.txt");

    if !path::Path::new(cache_folder).exists() {
        fs::create_dir(cache_folder).ok();
    }

    if !path::Path::new(file_path.as_str()).is_file() {
        let url = format!("https://adventofcode.com/2023/day/{day}/input");
        let input = download_file(url);
        let mut file = fs::File::create(file_path).unwrap();
        let mut bytes = input.as_bytes();
        io::copy(&mut bytes, &mut file).ok();
        return input;
    }

    return fs::read_to_string(file_path.as_str()).unwrap();
}
