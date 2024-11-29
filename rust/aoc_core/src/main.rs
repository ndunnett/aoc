use std::{
    env, fs,
    io::Write,
    process::{self, Stdio},
};

use clap::Parser;

use aoc_core::{
    error::err_blank,
    input::{delete_cached_input, load_input},
    Anyhow, AOC_URL,
};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Year of the puzzle to select
    #[arg(short, long, value_parser = clap::value_parser!(u16).range(2000..3000))]
    year: u16,

    /// Day of the puzzle to select
    #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..=25))]
    day: u8,

    /// Part of the puzzle to select (omit to run all parts)
    #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..=2))]
    part: Option<u8>,

    /// Run solution on selected puzzle
    #[arg(short, long, group = "exec")]
    run: bool,

    /// Run in release mode
    #[arg(long, requires = "exec")]
    release: bool,

    /// Run tests for selected puzzle
    #[arg(short, long, group = "exec")]
    test: bool,

    /// Print puzzle input
    #[arg(short, long)]
    input: bool,

    /// Start new puzzle from template
    #[arg(short, long)]
    new: bool,

    /// Force redownloading input and overwriting cache
    #[arg(long)]
    no_cache: bool,

    /// Open puzzle page in browser
    #[arg(short, long)]
    open: bool,
}

fn main() -> Anyhow<()> {
    let cli = Cli::parse();

    if cli.no_cache {
        delete_cached_input(cli.year, cli.day)?;
    }

    if cli.run || (!cli.new && !cli.input && !cli.open) {
        run(cli.year, cli.day, cli.part, cli.release, false)?;
    }

    if cli.test {
        run(cli.year, cli.day, cli.part, cli.release, true)?;
    }

    if cli.new {
        make_new(cli.year, cli.day)?;
    }

    if cli.input {
        print!("{}", load_input(cli.year, cli.day)?);
    }

    if cli.open {
        let url = format!("{AOC_URL}/{}/day/{}", cli.year, cli.day);
        webbrowser::open(&url)?;
    }

    Ok(())
}

fn run(year: u16, day: u8, part: Option<u8>, release: bool, test: bool) -> Anyhow<()> {
    let mut args = vec![
        (if test { "test" } else { "run" }).to_string(),
        "--package".to_string(),
        format!("aoc-{year}"),
        "--bin".to_string(),
        format!("day{day:02}"),
    ];

    if release {
        args.push("--release".to_string());
    }

    args.push("--".to_string());

    match part {
        Some(1) => args.push("--part1".to_string()),
        Some(2) => args.push("--part2".to_string()),
        _ => {
            args.push("--part1".to_string());
            args.push("--part2".to_string());
        }
    }

    let exit_status = process::Command::new("cargo")
        .args(&args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?
        .wait()?;

    if exit_status.success() {
        Ok(())
    } else {
        Err(err_blank())
    }
}

fn make_new(year: u16, day: u8) -> Anyhow<()> {
    let cwd = env::current_dir()?;
    let directory = cwd.join(format!("{year}/src/bin"));

    if !directory.is_dir() {
        fs::create_dir(&directory)?;
    }

    let path = directory.join(format!("day{day:02}.rs"));

    if path.exists() {
        let timestamp = chrono::Local::now().format("%F_%H.%M.%S");
        let backup_path = directory.join(format!("day{day:02}_{timestamp}.rs"));
        fs::copy(&path, backup_path)?;
    }

    let bytes = include_bytes!("template.rs");
    let mut file = fs::File::create(path)?;
    file.write_all(bytes)?;

    Ok(())
}
