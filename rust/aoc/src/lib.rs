pub use std::{
    collections::{HashMap, HashSet},
    fmt,
    sync::OnceLock,
};

pub use aoc_core::{error::err, input::auto_input, Anyhow};

pub use colored::Colorize;

pub trait Solver
where
    Self: std::marker::Sized,
{
    fn new(input: &str) -> Anyhow<Self>;
    fn part1(&mut self) -> Anyhow<impl fmt::Display>;
    fn part2(&mut self) -> Anyhow<impl fmt::Display>;
}

pub mod __runner {
    pub use paste::paste;

    pub fn format_time(time: std::time::Duration) -> super::Anyhow<String> {
        let s = format!("{time:#?}");

        let number = s
            .chars()
            .take_while(|c| c.is_ascii_digit() || c == &'.')
            .collect::<String>()
            .parse::<f64>()?;

        let shift = 10_f64.powi(4 - number.abs().log10().ceil() as i32);
        let rounded = (number * shift).round() / shift;
        let unit = s.chars().filter(|c| c.is_alphabetic()).collect::<String>();
        Ok(format!("{rounded} {unit}"))
    }

    pub const RUN_TIME: std::time::Duration = std::time::Duration::from_millis(250);
}

#[macro_export]
macro_rules! solution {
    () => {
        $crate::solution!(1, 2);
    };
    ($($part:tt),+) => {
        use $crate::*;

        fn main() -> Anyhow<()> {
            __runner::paste! {
                let input = auto_input(file!())?;

                let now = std::time::Instant::now();
                let mut solution = Solution::new(&input)?;
                let build_duration = now.elapsed();
                let mut total_duration = build_duration;

                for arg in std::env::args() {
                    match arg.as_str() {
                        $(
                            concat!("--part", stringify!($part)) => {
                                let now = std::time::Instant::now();
                                let answer = format!("{}", solution.[<part$part>]()?);
                                let mut duration = now.elapsed();
                                let mut i = 1;

                                while now.elapsed() < __runner::RUN_TIME {
                                    let now = std::time::Instant::now();
                                    solution.[<part$part>]()?;
                                    duration += now.elapsed();
                                    i += 1
                                }

                                let duration = now.elapsed().div_f64(i as f64);
                                total_duration += duration;

                                println!(
                                    concat!("Part ", stringify!($part), " answer: {} {}"),
                                    answer.bold().bright_blue(),
                                    format!("({})", __runner::format_time(duration)?).dimmed(),
                                );
                            }
                        )+
                        _ => {},
                    }
                }

                println!("{}",
                    format!("Build: {}  Total: {}",
                    __runner::format_time(build_duration)?,
                    __runner::format_time(total_duration)?,).dimmed()
                );

                Ok(())
            }
        }
    };
}
