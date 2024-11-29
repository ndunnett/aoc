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
        let re = Regex::new(r"\d+")?;

        Ok(self.lines.iter().fold(0, |acc, line| {
            let mut numbers = line.splitn(3, [':', '|']).skip(1).map(|s| {
                re.find_iter(s)
                    .map(|m| m.as_str().parse::<u32>().unwrap())
                    .collect()
            });

            let winners = numbers.next().unwrap();
            let picks: Vec<_> = numbers.last().unwrap();
            let matches = winners
                .iter()
                .filter(|winner| picks.contains(winner))
                .count() as u32;

            if matches > 0 {
                acc + 2_u32.pow(matches - 1)
            } else {
                acc
            }
        }))
    }

    fn part2(&mut self) -> Anyhow<impl fmt::Display> {
        let re = Regex::new(r"\d+")?;
        let mut copy_counts: HashMap<usize, u32> = HashMap::new();

        Ok(self.lines.iter().enumerate().fold(0, |acc, (i, line)| {
            let mut numbers = line.splitn(3, [':', '|']).skip(1).map(|s| {
                re.find_iter(s)
                    .map(|m| m.as_str().parse::<u32>().unwrap())
                    .collect()
            });

            let winners = numbers.next().unwrap();
            let picks: Vec<_> = numbers.last().unwrap();
            let matches = winners
                .iter()
                .filter(|winner| picks.contains(winner))
                .count();

            let n = copy_counts.get(&i).unwrap_or(&0) + 1;

            if matches > 0 {
                for j in 1..=matches {
                    copy_counts
                        .entry(i + j)
                        .and_modify(|count| *count += n)
                        .or_insert(n);
                }
            }

            acc + n
        }))
    }
}

aoc::solution!();
