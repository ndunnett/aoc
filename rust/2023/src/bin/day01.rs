use regex::Regex;

fn parse_caps(m: &str) -> u32 {
    match m {
        "one" | "1" => 1,
        "two" | "2" => 2,
        "three" | "3" => 3,
        "four" | "4" => 4,
        "five" | "5" => 5,
        "six" | "6" => 6,
        "seven" | "7" => 7,
        "eight" | "8" => 8,
        "nine" | "9" => 9,
        _ => panic!("invalid match"),
    }
}

fn rev(input: &str) -> String {
    input.chars().rev().collect::<String>()
}

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
        let re = Regex::new(r"\d")?;

        for line in self.lines.iter() {
            let numbers: Vec<u32> = re
                .find_iter(line)
                .map(|m| m.as_str().parse().unwrap())
                .collect();
            sum += numbers[0] * 10 + numbers[numbers.len() - 1];
        }

        Ok(sum)
    }

    fn part2(&mut self) -> Anyhow<impl fmt::Display> {
        let mut sum = 0;
        let pattern = "one|two|three|four|five|six|seven|eight|nine";
        let re_first = Regex::new(format!("(\\d|{})", pattern).as_str())?;
        let re_last = Regex::new(format!("(\\d|{})", rev(pattern)).as_str())?;

        for line in self.lines.iter() {
            // disgusting
            let first = parse_caps(re_first.captures(line).unwrap().get(0).unwrap().as_str());
            let last = parse_caps(
                rev(re_last
                    .captures(rev(line).as_str())
                    .unwrap()
                    .get(0)
                    .unwrap()
                    .as_str())
                .as_str(),
            );
            sum += first * 10 + last;
        }

        Ok(sum)
    }
}

aoc::solution!();
