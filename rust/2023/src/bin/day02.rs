use std::cmp::max;

use regex::Regex;

pub struct Solution {
    lines: Vec<String>,
}

impl Solver for Solution {
    fn new(input: &str) -> Anyhow<Self> {
        Ok(Self {
            lines: input.lines().map(String::from).collect(),
        })
    }

    fn part1(&mut self) -> Anyhow<impl fmt::Display> {
        let mut sum = 0;
        let game_re = Regex::new(r"Game (?P<num>\d+):")?;

        let exprs = vec![
            (Regex::new(r"(?P<num>\d+) red")?, 12),
            (Regex::new(r"(?P<num>\d+) green")?, 13),
            (Regex::new(r"(?P<num>\d+) blue")?, 14),
        ];

        for line in self.lines.iter() {
            let game = game_re
                .captures(line)
                .unwrap()
                .name("num")
                .unwrap()
                .as_str()
                .parse::<u32>()?;

            let mut possible = true;

            'outer: for (re, cubes) in &exprs {
                for cap in re.captures_iter(line) {
                    if cap.name("num").unwrap().as_str().parse::<u32>()? > *cubes {
                        possible = false;
                        break 'outer;
                    }
                }
            }

            if possible {
                sum += game;
            }
        }

        Ok(sum)
    }

    fn part2(&mut self) -> Anyhow<impl fmt::Display> {
        let mut sum = 0;

        let exprs = vec![
            Regex::new(r"(?P<num>\d+) red")?,
            Regex::new(r"(?P<num>\d+) green")?,
            Regex::new(r"(?P<num>\d+) blue")?,
        ];

        for line in self.lines.iter() {
            let mut power = 1;

            for re in &exprs {
                let mut cubes = 0;

                for cap in re.captures_iter(line) {
                    cubes = max(cubes, cap.name("num").unwrap().as_str().parse::<u32>()?);
                }

                power *= cubes;
            }

            sum += power;
        }

        Ok(sum)
    }
}

aoc::solution!();
