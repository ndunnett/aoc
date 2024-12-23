fn encode(n: u32) -> u32 {
    // this function is a simplification of operations:
    //
    // n ^= n * 64    --> operand is base 2, can be done by bit shifting left by log2(64) == 6
    // n %= 16777216  --> 16777216 == 0x1000000, can be done by bit masking 0xFFFFFF
    // n ^= n / 32    --> operand is base 2, can be done by bit shifting right by log2(32) == 5
    // n %= 16777216  --> number of bits will never increase after division, modulo not needed
    // n ^= n * 2048  --> operand is base 2, can be done by bit shifting left by log2(2048) == 11
    // n %= 16777216

    let a = (n ^ (n << 6)) & 0xFFFFFF;
    let b = a ^ (a >> 5);
    (b ^ (b << 11)) & 0xFFFFFF
}

struct Solution {
    secrets: Vec<u32>,
}

impl Solver for Solution {
    fn new(input: &str) -> Anyhow<Self> {
        Ok(Self {
            secrets: input
                .lines()
                .map(str::parse::<u32>)
                .collect::<ParseIntResult<Vec<_>>>()?,
        })
    }

    fn part1(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self
            .secrets
            .iter()
            .map(|&secret| {
                let mut result = secret;

                for _ in 0..2000 {
                    result = encode(result);
                }

                result as u64
            })
            .sum::<u64>())
    }

    fn part2(&mut self) -> Anyhow<impl fmt::Display> {
        // store state in vectors instead of hashmaps/hashsets
        let mut costs = vec![0; 0xFFFFF];
        let mut seen = vec![0; 0xFFFFF];

        for &secret in &self.secrets {
            let mut result = secret;
            let mut previous_cost = result % 10;
            let mut deltas = 0;
            let mut deltas_filled = 0b11111;

            for _ in 0..2000 {
                result = encode(result);
                let cost = result % 10;

                // offset cost delta by +10 and represent as 5 bit unsigned int (max == 19 == 0b10101)
                // store sliding window of 4 deltas as a 20 bit unsigned int (max == 0xFFFFF)
                deltas = ((deltas << 5) & 0xFFFFF) + 10 + cost - previous_cost;

                // start checking prices once deltas window is populated
                // only counting the first occurance of each unique delta sequence
                if deltas_filled < 0xFFFFF {
                    deltas_filled = (deltas_filled << 5) + 0b11111;
                } else if seen[deltas as usize] != secret {
                    seen[deltas as usize] = secret;
                    costs[deltas as usize] += cost;
                }

                previous_cost = cost;
            }
        }

        costs
            .iter()
            .max()
            .cloned()
            .ok_or(anyhow!("failed to find best sequence"))
    }
}

aoc::solution!();

#[cfg(test)]
mod test {
    use super::{Solution, Solver};

    const INPUT1: &str = r"1
10
100
2024";

    const INPUT2: &str = r"1
2
3
2024";

    #[test]
    fn test_part1() {
        let mut solution = Solution::new(INPUT1).unwrap();
        let answer = solution.part1().unwrap().to_string();
        assert_eq!(answer, "37327623");
    }

    #[test]
    fn test_part2() {
        let mut solution = Solution::new(INPUT2).unwrap();
        let answer = solution.part2().unwrap().to_string();
        assert_eq!(answer, "23");
    }
}
