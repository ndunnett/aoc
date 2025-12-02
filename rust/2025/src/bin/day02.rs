#[derive(Debug, Clone)]
struct IdRange {
    start: isize,
    end: isize,
}

#[derive(Clone)]
struct Solution {
    ranges: Vec<IdRange>,
}

impl Solution {
    fn solve(id: isize, digits: u32) -> bool {
        let base = 10_isize.pow(digits);
        let pattern = id % base;

        if pattern == 0 {
            return false;
        }

        let mut rem = id;

        while rem >= pattern {
            rem -= pattern;

            if rem >= base {
                rem /= base;

                if rem % base != pattern {
                    break;
                }
            } else {
                break;
            }
        }

        if rem == 0 {
            println!("{id} -> {base} -> {pattern}");
        }

        rem == 0
    }
}

impl Solver for Solution {
    fn new(input: &str) -> Anyhow<Self> {
        Ok(Self {
            ranges: NumberParser::<isize>::from(input)
                .tuples()
                .map(|(start, end)| IdRange { start, end })
                .collect(),
        })
    }

    fn part1(&mut self) -> Anyhow<impl fmt::Display> {
        let mut count = 0;

        for range in &self.ranges {
            for id in range.start..=range.end {
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

        for range in &self.ranges {
            for id in range.start..=range.end {
                let total = id.ilog10() + 1;

                for digits in 1..=id.ilog10().div_ceil(2) {
                    if total % digits == 0 && Self::solve(id, digits) {
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
