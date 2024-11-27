aoc::solution!();

use itertools::Itertools;

struct Point {
    x: usize,
    y: usize,
}

type Universe = Vec<Point>;

trait Solution {
    fn parse(input: &str) -> Universe;
    fn expanded(&self, age: usize) -> Universe;
    fn solve(&self) -> usize;
}

impl Solution for Universe {
    fn parse(input: &str) -> Universe {
        input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(|(x, c)| if c == '#' { Some(Point { x, y }) } else { None })
                    .collect_vec()
            })
            .collect_vec()
    }

    fn expanded(&self, age: usize) -> Universe {
        let extent = self.iter().fold(Point { x: 0, y: 0 }, |acc, g| Point {
            x: acc.x.max(g.x),
            y: acc.y.max(g.y),
        });

        let cols = (0..=extent.x).filter(|x| self.iter().all(|g| g.x != *x));
        let rows = (0..=extent.y).filter(|y| self.iter().all(|g| g.y != *y));

        self.iter()
            .map(|g| Point {
                x: g.x + age.saturating_sub(1) * cols.clone().filter(|x| x < &g.x).count(),
                y: g.y + age.saturating_sub(1) * rows.clone().filter(|y| y < &g.y).count(),
            })
            .collect_vec()
    }

    fn solve(&self) -> usize {
        self.iter()
            .combinations(2)
            .map(|v| v[0].x.abs_diff(v[1].x) + v[0].y.abs_diff(v[1].y))
            .sum()
    }
}

fn part1(input: &str) -> usize {
    Universe::parse(input).expanded(2).solve()
}

fn part2(input: &str) -> usize {
    Universe::parse(input).expanded(1000000).solve()
}

#[cfg(test)]
mod test {
    use super::{Solution, Universe};

    const INPUT1: &str = "...#......
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
        assert_eq!(Universe::parse(INPUT1).expanded(2).solve(), 374);
    }

    #[test]
    fn test_part2() {
        assert_eq!(Universe::parse(INPUT1).expanded(10).solve(), 1030);
        assert_eq!(Universe::parse(INPUT1).expanded(100).solve(), 8410);
    }
}
