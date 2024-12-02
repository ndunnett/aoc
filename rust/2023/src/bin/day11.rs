#[derive(Clone, Copy, PartialEq)]
struct Point {
    pub x: usize,
    pub y: usize,
}

impl From<(usize, usize)> for Point {
    fn from(tuple: (usize, usize)) -> Self {
        Self {
            x: tuple.0,
            y: tuple.1,
        }
    }
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

struct Solution {
    universe: Vec<Point>,
}

impl Solution {
    fn expanded(&self, age: usize) -> Self {
        let rate = age.saturating_sub(1);

        let x_extent = self
            .universe
            .iter()
            .fold(0, |acc, galaxy| acc.max(galaxy.x));

        let y_extent = self.universe[self.universe.len() - 1].y;

        let empty_cols = (0..=x_extent)
            .into_par_iter()
            .filter(|x| self.universe.iter().all(|g| g.x != *x))
            .collect::<Vec<_>>();

        let empty_rows = (0..=y_extent)
            .into_par_iter()
            .filter(|y| self.universe.iter().all(|g| g.y != *y))
            .collect::<Vec<_>>();

        let universe = self
            .universe
            .par_iter()
            .map(|galaxy| {
                Point::new(
                    galaxy.x + rate * empty_cols.iter().filter(|&&x| x < galaxy.x).count(),
                    galaxy.y + rate * empty_rows.iter().filter(|&&y| y < galaxy.y).count(),
                )
            })
            .collect();

        Self { universe }
    }

    fn solve(&self) -> usize {
        (0..self.universe.len() - 1)
            .into_par_iter()
            .flat_map_iter(|a| {
                (a + 1..self.universe.len()).map(move |b| (self.universe[a], self.universe[b]))
            })
            .map(|(a, b)| a.x.abs_diff(b.x) + a.y.abs_diff(b.y))
            .sum()
    }
}

impl Solver for Solution {
    fn new(input: &str) -> Anyhow<Self> {
        let universe = input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(|(x, c)| {
                        if c == '#' {
                            Some(Point::new(x, y))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        Ok(Self { universe })
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
