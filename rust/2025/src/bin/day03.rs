#[derive(Clone)]
struct Solution {
    banks: Vec<Vec<u8>>,
}

impl Solution {
    fn solve(&self, batteries: usize) -> u64 {
        let mut joltage = 0;

        for bank in &self.banks {
            let mut last_best = None;

            for n in (0..batteries).rev() {
                let mut best = last_best.map_or(0, |n| n + 1);
                let mut value = bank[best];
                let mut i = best + 1;

                while i < bank.len() - n && value < 9 {
                    let current = bank[i];

                    if current > value {
                        best = i;
                        value = current;
                    }

                    i += 1;
                }

                last_best = Some(best);
                joltage += 10u64.pow(n as u32) * value as u64;
            }
        }

        joltage
    }
}

impl Solver for Solution {
    fn new(input: &str) -> Anyhow<Self> {
        Ok(Self {
            banks: input
                .lines()
                .map(|line| line.bytes().map(|b| b - b'0').collect())
                .collect(),
        })
    }

    fn part1(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self.solve(2))
    }

    fn part2(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self.solve(12))
    }
}

aoc::solution!();

#[cfg(test)]
mod test {
    use super::{Solution, Solver};

    const INPUT: &str = r"987654321111111
811111111111119
234234234234278
818181911112111
";

    #[test]
    fn test_part1() {
        let mut solution = Solution::new(INPUT).unwrap();
        let answer = solution.part1().unwrap().to_string();
        assert_eq!(answer, "357");
    }

    #[test]
    fn test_part2() {
        let mut solution = Solution::new(INPUT).unwrap();
        let answer = solution.part2().unwrap().to_string();
        assert_eq!(answer, "3121910778619");
    }
}
