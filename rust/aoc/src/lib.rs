pub use std::{
    collections::{HashMap, HashSet},
    fmt,
    sync::OnceLock,
};

pub type ParseIntResult<T> = Result<T, std::num::ParseIntError>;
pub type ParseFloatResult<T> = Result<T, std::num::ParseFloatError>;

pub use anyhow::{anyhow, Error};
pub use colored::Colorize;
pub use itertools::{self, Itertools};
pub use rayon::prelude::*;
pub use regex::{self, Regex};
pub use rustc_hash::{FxBuildHasher, FxHashMap, FxHashSet, FxHasher};

pub type Anyhow<T> = anyhow::Result<T>;

pub trait Solver
where
    Self: std::marker::Sized,
{
    fn new(input: &str) -> Anyhow<Self>;
    fn part1(&mut self) -> Anyhow<impl fmt::Display>;
    fn part2(&mut self) -> Anyhow<impl fmt::Display>;
}

pub mod __runner {
    pub use aoc_core::auto_input;
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

    pub fn generalise_durations(durations: &[std::time::Duration]) -> std::time::Duration {
        let partition = &durations[durations.len() / 4..];

        partition
            .iter()
            .sum::<std::time::Duration>()
            .div_f64(partition.len() as f64)
    }

    pub const RUN_TIME: std::time::Duration = std::time::Duration::from_millis(1000);
}

/// Generates a main function for the runner to call and includes commonly used imports.
///
/// Standard library imports:
/// ```
/// pub use std::{
///     collections::{HashMap, HashSet},
///     fmt,
///     sync::OnceLock,
/// };
/// ```
///
/// Third party imports:
/// ```
/// pub use anyhow::{anyhow, Error};
/// pub use colored::Colorize;
/// pub use itertools::{self, Itertools};
/// pub use rayon::prelude::*;
/// pub use regex::{self, Regex};
/// pub use rustc_hash::{FxHashMap, FxHashSet, FxHasher, FxBuildHasher};
/// ```
///
/// Type definitions:
/// ```
/// pub type ParseIntResult<T> = Result<T, std::num::ParseIntError>;
/// pub type ParseFloatResult<T> = Result<T, std::num::ParseFloatError>;
/// pub type Anyhow<T> = anyhow::Result<T>;
/// ```
#[macro_export]
macro_rules! solution {
    () => {
        $crate::solution!(1, 2);
    };
    ($($part:tt),+) => {
        use $crate::*;

        fn main() -> Anyhow<()> {
            __runner::paste! {
                let input = __runner::auto_input(file!())?;

                let now = std::time::Instant::now();
                let mut solution = Solution::new(&input)?;
                let mut build_durations = vec![now.elapsed()];

                while now.elapsed() < __runner::RUN_TIME {
                    let now = std::time::Instant::now();
                    solution= Solution::new(&input)?;
                    build_durations.push(now.elapsed());
                }

                let build_duration = __runner::generalise_durations(&build_durations);
                let mut total_duration = build_duration;

                for arg in std::env::args() {
                    match arg.as_str() {
                        $(
                            concat!("--part", stringify!($part)) => {
                                let now = std::time::Instant::now();
                                let answer = solution.[<part$part>]()?.to_string();
                                let mut solution_durations = vec![now.elapsed()];

                                while now.elapsed() < __runner::RUN_TIME {
                                    let now = std::time::Instant::now();
                                    let next_answer = solution.[<part$part>]()?;
                                    solution_durations.push(now.elapsed());

                                    if next_answer.to_string() != answer {
                                        return Err(anyhow!("inconsistent result"));
                                    }
                                }

                                let solution_duration = __runner::generalise_durations(&solution_durations);
                                total_duration += solution_duration;

                                println!(
                                    concat!("Part ", stringify!($part), " answer: {} {}"),
                                    format!("{}", answer.to_string().bold().bright_blue()),
                                    format!("({})", __runner::format_time(solution_duration)?).dimmed(),
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
