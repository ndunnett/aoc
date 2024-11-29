use itertools::Itertools;

struct Point {
    x: usize,
    y: usize,
}

pub struct Solution {
    uni: Vec<Point>,
}

impl Solution {
    fn expanded(&self, age: usize) -> Self {
        let extent = self.uni.iter().fold(Point { x: 0, y: 0 }, |acc, g| Point {
            x: acc.x.max(g.x),
            y: acc.y.max(g.y),
        });

        let cols = (0..=extent.x).filter(|x| self.uni.iter().all(|g| g.x != *x));
        let rows = (0..=extent.y).filter(|y| self.uni.iter().all(|g| g.y != *y));

        let uni = self
            .uni
            .iter()
            .map(|g| Point {
                x: g.x + age.saturating_sub(1) * cols.clone().filter(|x| x < &g.x).count(),
                y: g.y + age.saturating_sub(1) * rows.clone().filter(|y| y < &g.y).count(),
            })
            .collect_vec();

        Self { uni }
    }

    fn solve(&self) -> usize {
        self.uni
            .iter()
            .combinations(2)
            .map(|v| v[0].x.abs_diff(v[1].x) + v[0].y.abs_diff(v[1].y))
            .sum()
    }
}

impl Solver for Solution {
    fn new(input: &str) -> Anyhow<Self> {
        let uni = input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(|(x, c)| if c == '#' { Some(Point { x, y }) } else { None })
                    .collect_vec()
            })
            .collect_vec();

        Ok(Self { uni })
    }

    fn part1(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self.expanded(2).solve())
    }

    fn part2(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self.expanded(1000000).solve())
    }
}

aoc::solution!();

#[cfg(test)]
mod test {
    use super::{Solution, Solver};

    const INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn test_part1() {
        let solution = Solution::new(INPUT).unwrap();
        let answer = solution.expanded(2).solve();
        assert_eq!(answer, 374);
    }

    #[test]
    fn test_part2_1() {
        let solution = Solution::new(INPUT).unwrap();
        let answer = solution.expanded(10).solve();
        assert_eq!(answer, 1030);
    }

    #[test]
    fn test_part2_2() {
        let solution = Solution::new(INPUT).unwrap();
        let answer = solution.expanded(100).solve();
        assert_eq!(answer, 8410);
    }
}
