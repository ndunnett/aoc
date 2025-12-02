#[derive(Clone)]
struct Solution {
    ranges: Vec<(u64, u64)>,
}

impl Solution {
    #[inline(always)]
    fn solve(mut id: u64, digits: u32) -> bool {
        let base = 10_u64.pow(digits);
        let pattern = id % base;

        if pattern == 0 {
            return false;
        }

        while id >= base {
            if id % base != pattern {
                return false;
            }

            id /= base;
        }

        id == pattern
    }
}

impl Solver for Solution {
    fn new(input: &str) -> Anyhow<Self> {
        Ok(Self {
            ranges: NumberParser::<u64>::from(input).tuples().collect(),
        })
    }

    fn part1(&mut self) -> Anyhow<impl fmt::Display> {
        let mut count = 0;

        for &(first, last) in &self.ranges {
            for id in first..=last {
                let digits = id.ilog10() + 1;

                if digits % 2 == 0 && Self::solve(id, digits / 2) {
                    count += id;
                }
            }
        }

        Ok(count)
    }

    fn part2(&mut self) -> Anyhow<impl fmt::Display> {
        let mut count = 0;

        for &(first, last) in &self.ranges {
            for id in first..=last {
                let digits = id.ilog10() + 1;

                for d in 1..=((digits) >> 1) {
                    if digits % d == 0 && Self::solve(id, d) {
                        count += id;
                        break;
                    }
                }
            }
        }

        Ok(count)
    }
}

aoc::solution!();

#[cfg(test)]
mod test {
    use super::{Solution, Solver};

    const INPUT: &str = r"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124
";

    #[test]
    fn test_part1() {
        let mut solution = Solution::new(INPUT).unwrap();
        let answer = solution.part1().unwrap().to_string();
        assert_eq!(answer, "1227775554");
    }

    #[test]
    fn test_part2() {
        let mut solution = Solution::new(INPUT).unwrap();
        let answer = solution.part2().unwrap().to_string();
        assert_eq!(answer, "4174379265");
    }
}
