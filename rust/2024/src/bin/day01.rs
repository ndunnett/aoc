struct Solution {
    a: Vec<usize>,
    b: Vec<usize>,
}

impl Solution {
    const SPLIT_PATTERN: [char; 2] = [' ', '\n'];
}

impl Solver for Solution {
    fn new(input: &str) -> Anyhow<Self> {
        let (mut a, mut b): (Vec<_>, Vec<_>) = input
            .split(Self::SPLIT_PATTERN)
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<usize>().expect("failed to parse int"))
            .tuples::<(_, _)>()
            .unzip();

        a.sort_unstable();
        b.sort_unstable();

        Ok(Self { a, b })
    }

    fn part1(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self
            .a
            .iter()
            .zip(self.b.iter())
            .fold(0, |acc, (a, b)| acc + a.abs_diff(*b)))
    }

    fn part2(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self.a.iter().fold(0, |acc, a| {
            acc + self.b.iter().filter(|&b| a == b).count() * a
        }))
    }
}

aoc::solution!();

#[cfg(test)]
mod test {
    use super::{Solution, Solver};

    const INPUT: &str = r"3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test_part1() {
        let mut solution = Solution::new(INPUT).unwrap();
        let answer = solution.part1().unwrap().to_string();
        assert_eq!(answer, "11");
    }

    #[test]
    fn test_part2() {
        let mut solution = Solution::new(INPUT).unwrap();
        let answer = solution.part2().unwrap().to_string();
        assert_eq!(answer, "31");
    }
}
