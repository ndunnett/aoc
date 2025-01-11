#[derive(Clone)]
struct Solution {
    a: Vec<u32>,
    b: Vec<u32>,
}

impl Solver for Solution {
    fn new(input: &str) -> Anyhow<Self> {
        let (a, b) = NumberParser::<u32>::from(input).tuples().unzip();
        Ok(Self { a, b })
    }

    fn part1(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self
            .a
            .iter()
            .sorted_unstable()
            .zip(self.b.iter().sorted_unstable())
            .map(|(a, b)| a.abs_diff(*b))
            .sum::<u32>())
    }

    fn part2(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self
            .a
            .par_iter()
            .map(|a| a * self.b.iter().filter(|&b| a == b).count() as u32)
            .sum::<u32>())
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
