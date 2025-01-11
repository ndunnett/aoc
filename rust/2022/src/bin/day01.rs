#[derive(Clone)]
struct Solution {
    reindeer: Vec<u32>,
}

impl Solver for Solution {
    fn new(input: &str) -> Anyhow<Self> {
        let mut reindeer = Vec::new();
        let mut calories = 0;

        for line in input.lines() {
            if line.is_empty() {
                reindeer.push(calories);
                calories = 0;
            } else {
                calories += line.parse::<u32>()?;
            }
        }

        Ok(Self { reindeer })
    }

    fn part1(&mut self) -> Anyhow<impl fmt::Display> {
        self.reindeer.iter().max().ok_or(anyhow!("empty input"))
    }

    fn part2(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self.reindeer.iter().take(3).sum::<u32>())
    }
}

aoc::solution!();
