use regex::Regex;

fn get_edges(i: usize, j: usize) -> [(usize, usize); 8] {
    [
        (i.saturating_sub(1), j.saturating_sub(1)),
        (i.saturating_sub(1), j),
        (i.saturating_sub(1), j + 1),
        (i, j.saturating_sub(1)),
        (i, j + 1),
        (i + 1, j.saturating_sub(1)),
        (i + 1, j),
        (i + 1, j + 1),
    ]
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
        let symbol_re = Regex::new(r"[^\d.]")?;
        let number_re = Regex::new(r"\d+")?;

        let symbols_vec: Vec<Vec<usize>> = self
            .lines
            .iter()
            .map(|line| symbol_re.find_iter(line).map(|m| m.start()).collect())
            .collect();

        for (i, line) in self.lines.iter().enumerate() {
            sum += number_re
                .find_iter(line)
                .filter(|m| {
                    symbols_vec
                        .iter()
                        .skip(i.saturating_sub(1))
                        .take(3)
                        .any(|symbols| {
                            symbols
                                .iter()
                                .any(|&pos| (m.start().saturating_sub(1)..=m.end()).contains(&pos))
                        })
                })
                .map(|m| m.as_str().parse::<u32>().unwrap())
                .sum::<u32>();
        }

        Ok(sum)
    }

    fn part2(&mut self) -> Anyhow<impl fmt::Display> {
        let mut sum = 0;
        let gear_re = Regex::new(r"\*")?;
        let number_re = Regex::new(r"\d+")?;

        let numbers_vec = self
            .lines
            .iter()
            .map(|line| {
                number_re
                    .find_iter(line)
                    .map(|m| (m.range(), m.as_str().parse::<u32>().unwrap()))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        for (i, line) in self.lines.iter().enumerate() {
            for m in gear_re.find_iter(line) {
                let edges = get_edges(i, m.start());
                let gear_numbers = edges
                    .iter()
                    .flat_map(|(x, y)| {
                        numbers_vec.get(*x).and_then(|vec| {
                            vec.iter()
                                .find(|(range, _)| *y >= range.start && *y < range.end)
                                .map(|(_, number)| *number)
                        })
                    })
                    .collect::<HashSet<_>>();

                if gear_numbers.len() == 2 {
                    sum += gear_numbers.iter().product::<u32>();
                }
            }
        }

        Ok(sum)
    }
}

aoc::solution!();
