fn extrapolate(history: Vec<i32>) -> i32 {
    let mut components = vec![history];

    while let Some(last) = components.last() {
        if last.iter().all_equal() {
            break;
        }

        components.push(last.windows(2).map(|p| p[1] - p[0]).collect());
    }

    components
        .iter()
        .map(|c| c.last().expect("failed to extrapolate line"))
        .sum()
}

#[derive(Clone)]
struct Solution {
    history: Vec<Vec<i32>>,
}

impl Solver for Solution {
    fn new(input: &str) -> Anyhow<Self> {
        Ok(Self {
            history: input
                .lines()
                .map(|line| {
                    line.split(' ')
                        .map(|s| s.parse())
                        .collect::<ParseIntResult<Vec<_>>>()
                })
                .collect::<ParseIntResult<Vec<_>>>()?,
        })
    }

    fn part1(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self
            .history
            .par_iter()
            .map(|h| extrapolate(h.to_vec()))
            .sum::<i32>())
    }

    fn part2(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self
            .history
            .par_iter()
            .map(|h| extrapolate(h.iter().rev().cloned().collect()))
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
