use itertools::Itertools;

fn decompose(mut components: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let last = components.last().unwrap();

    if last.iter().all_equal() {
        return components;
    }

    components.push(last.windows(2).map(|p| p[1] - p[0]).collect_vec());
    decompose(components)
}

fn predict(history: Vec<i32>) -> i32 {
    decompose(vec![history])
        .iter()
        .map(|c| c.last().unwrap())
        .sum()
}

pub struct Solution {
    grid: Vec<Vec<i32>>,
}

impl Solver for Solution {
    fn new(input: &str) -> Anyhow<Self> {
        let grid = input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect()
            })
            .collect();

        Ok(Self { grid })
    }

    fn part1(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self.grid.iter().map(|h| predict(h.to_vec())).sum::<i32>())
    }

    fn part2(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self
            .grid
            .iter()
            .map(|h| predict(h.iter().rev().cloned().collect()))
            .sum::<i32>())
    }
}

aoc::solution!();

#[cfg(test)]
mod test {
    use super::{Solution, Solver};

    const INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn test_part1() {
        let mut solution = Solution::new(INPUT).unwrap();
        let answer = solution.part1().unwrap().to_string();
        assert_eq!(answer, "114");
    }

    #[test]
    fn test_part2() {
        let mut solution = Solution::new(INPUT).unwrap();
        let answer = solution.part2().unwrap().to_string();
        assert_eq!(answer, "2");
    }
}
