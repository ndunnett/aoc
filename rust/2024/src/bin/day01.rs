struct Solution {
    a: Vec<usize>,
    b: Vec<usize>,
}

impl Solver for Solution {
    fn new(input: &str) -> Anyhow<Self> {
        let (mut a, mut b): (Vec<_>, Vec<_>) = input
            .lines()
            .map(|line| {
                let (a, b) = line.split_once("   ").expect("failed to split line");

                (
                    a.parse::<usize>().expect("failed to parse int"),
                    b.parse::<usize>().expect("failed to parse int"),
                )
            })
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
        let b_counts = self
            .b
            .par_chunk_by(|&a, &b| a == b)
            .map(|chunk| (chunk[0], chunk.len()))
            .collect::<HashMap<usize, usize>>();

        Ok(self
            .a
            .par_iter()
            .map(|a| b_counts.get(a).unwrap_or(&0) * a)
            .sum::<usize>())
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
