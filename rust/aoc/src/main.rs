use std::process::{Command, Stdio};

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Year of the puzzle to select
    #[arg(short, long, value_parser = clap::value_parser!(u16).range(2000..3000))]
    year: u16,

    /// Day of the puzzle to select
    #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..=25))]
    day: u8,

    /// Build in release mode
    #[arg(long, default_value_t = false)]
    release: bool,
}

fn main() {
    let args = Args::parse();

    let mut cmd_args = vec![
        "run".to_string(),
        "--package".to_string(),
        format!("aoc-{}", args.year),
        "--bin".to_string(),
        format!("day{:02}", args.day),
    ];

    if args.release {
        cmd_args.push("--release".to_string());
    }

    let mut cmd = Command::new("cargo")
        .args(&cmd_args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();

    cmd.wait().unwrap();
}
