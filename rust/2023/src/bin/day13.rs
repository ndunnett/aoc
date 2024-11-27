aoc::solution!();

#[derive(Debug, Clone, Copy)]
enum Axis {
    V,
    H,
}

impl Axis {
    fn inverse(&self) -> Axis {
        match self {
            Axis::V => Axis::H,
            Axis::H => Axis::V,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Point {
        Point { x, y }
    }

    fn pivoted(&self, axis: Axis) -> Point {
        match axis {
            Axis::V => *self,
            Axis::H => Point::new(self.y, self.x),
        }
    }

    fn view(&self, axis: Axis) -> usize {
        match axis {
            Axis::V => self.x,
            Axis::H => self.y,
        }
    }
}

type MirrorView = Vec<usize>;

trait Reflection {
    fn find_reflection(&self, smudges: u32) -> Option<usize>;
}

impl Reflection for MirrorView {
    fn find_reflection(&self, smudges: u32) -> Option<usize> {
        self.windows(2)
            .enumerate()
            .filter_map(|(i, w)| {
                if (w[0] ^ w[1]).count_ones() <= smudges {
                    Some(i + 1)
                } else {
                    None
                }
            })
            .find(|&r| {
                self[..r]
                    .iter()
                    .rev()
                    .zip(self[r..].iter())
                    .fold(0, |acc, (&a, &b)| acc + (a ^ b).count_ones())
                    == smudges
            })
    }
}

type Mirror = Vec<Point>;

trait Solution {
    fn parse(input: &str) -> Mirror;
    fn view(&self, axis: Axis) -> MirrorView;
}

impl Solution for Mirror {
    fn parse(chunk: &str) -> Mirror {
        chunk
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().filter_map(move |(x, c)| {
                    if c == '#' {
                        Some(Point::new(x, y))
                    } else {
                        None
                    }
                })
            })
            .collect()
    }

    fn view(&self, axis: Axis) -> MirrorView {
        let extent = self.iter().fold(Point::new(0, 0), |acc, r| {
            Point::new(acc.x.max(r.x), acc.y.max(r.y))
        });

        (0..=extent.view(axis))
            .map(|a| {
                (0..=extent.view(axis.inverse())).fold(0, |acc, b| {
                    acc | ((self.contains(&Point::new(a, b).pivoted(axis)) as usize) << b)
                })
            })
            .collect()
    }
}

fn solve(input: &str, smudges: u32) -> usize {
    input
        .split("\n\n")
        .map(Mirror::parse)
        .fold(0, |acc, mirror| {
            acc + mirror
                .view(Axis::V)
                .find_reflection(smudges)
                .or(mirror
                    .view(Axis::H)
                    .find_reflection(smudges)
                    .map(|r| r * 100))
                .unwrap_or(0)
        })
}

fn part1(input: &str) -> usize {
    solve(input, 0)
}

fn part2(input: &str) -> usize {
    solve(input, 1)
}

#[cfg(test)]
mod test {
    use super::{part1, part2};

    const INPUT1: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT1), 405);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT1), 400);
    }
}
