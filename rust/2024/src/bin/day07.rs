struct Equation {
    value: u64,
    operands: Vec<u64>,
}

impl TryFrom<&str> for Equation {
    type Error = Error;

    fn try_from(line: &str) -> Result<Self, Self::Error> {
        let (value, operands) = line
            .split_once(": ")
            .ok_or(anyhow!("failed to split equation"))?;

        let value = value.parse::<u64>()?;

        let operands = operands
            .split(' ')
            .map(|s| s.parse::<u64>())
            .collect::<ParseIntResult<Vec<_>>>()?;

        Ok(Self { value, operands })
    }
}

impl Equation {
    fn is_correct(&self, n: &[u64], c: bool) -> bool {
        if n.len() == 2 {
            self.value == n[0] * n[1]
                || self.value == n[0] + n[1]
                || (c && self.value == concat(n[0], n[1]))
        } else {
            self.is_correct(&[[n[0] * n[1]].as_slice(), &n[2..]].concat(), c)
                || self.is_correct(&[[n[0] + n[1]].as_slice(), &n[2..]].concat(), c)
                || (c && self.is_correct(&[[concat(n[0], n[1])].as_slice(), &n[2..]].concat(), c))
        }
    }
}

fn concat(a: u64, b: u64) -> u64 {
    a * 10_u64.pow(b.ilog10() + 1) + b
}

struct Solution {
    equations: Vec<Equation>,
}

impl Solver for Solution {
    fn new(input: &str) -> Anyhow<Self> {
        Ok(Self {
            equations: input
                .lines()
                .map(Equation::try_from)
                .collect::<Anyhow<Vec<_>>>()?,
        })
    }

    fn part1(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self
            .equations
            .par_iter()
            .filter_map(|e| {
                if e.is_correct(&e.operands, false) {
                    Some(e.value)
                } else {
                    None
                }
            })
            .sum::<u64>())
    }

    fn part2(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self
            .equations
            .par_iter()
            .filter_map(|e| {
                if e.is_correct(&e.operands, true) {
                    Some(e.value)
                } else {
                    None
                }
            })
            .sum::<u64>())
    }
}

aoc::solution!();

#[cfg(test)]
mod test {
    use super::{Solution, Solver};

    const INPUT: &str = r"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_part1() {
        let mut solution = Solution::new(INPUT).unwrap();
        let answer = solution.part1().unwrap().to_string();
        assert_eq!(answer, "3749");
    }

    #[test]
    fn test_part2() {
        let mut solution = Solution::new(INPUT).unwrap();
        let answer = solution.part2().unwrap().to_string();
        assert_eq!(answer, "11387");
    }
}
