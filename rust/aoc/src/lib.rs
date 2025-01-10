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
pub use rustc_hash::{FxHashMap, FxHashSet, FxHasher, FxBuildHasher};

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
                let mut build_duration = now.elapsed();

                if build_duration < __runner::RUN_TIME {
                    build_duration = std::time::Duration::new(0, 0);
                    let mut i = 0;

                    while build_duration < __runner::RUN_TIME {
                        let now = std::time::Instant::now();
                        let _ = Solution::new(&input)?;
                        build_duration += now.elapsed();
                        i += 1
                    }

                    build_duration = build_duration.div_f64(i as f64);
                }

                let mut total_duration = build_duration;

                for arg in std::env::args() {
                    match arg.as_str() {
                        $(
                            concat!("--part", stringify!($part)) => {
                                let now = std::time::Instant::now();
                                let mut answer = solution.[<part$part>]()?.to_string();
                                let mut solution_duration = now.elapsed();

                                if solution_duration < __runner::RUN_TIME {
                                    solution_duration = now.elapsed();
                                    let mut i = 0;

                                    while solution_duration < __runner::RUN_TIME {
                                        let now = std::time::Instant::now();
                                        solution.[<part$part>]()?;
                                        solution_duration += now.elapsed();
                                        i += 1
                                    }

                                    solution_duration = solution_duration.div_f64(i as f64);
                                }

                                total_duration += solution_duration;

                                println!(
                                    concat!("Part ", stringify!($part), " answer: {} {}"),
                                    format!("{}", answer.bold().bright_blue()),
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
