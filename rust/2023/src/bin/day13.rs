#[derive(Clone, Copy)]
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

#[derive(Clone, Copy, Eq, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Point {
        Point { x, y }
    }

    fn pivoted(self, axis: Axis) -> Point {
        match axis {
            Axis::V => self,
            Axis::H => Point::new(self.y, self.x),
        }
    }

    fn view(self, axis: Axis) -> usize {
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
        self.windows(2).enumerate().find_map(|(i, w)| {
            if (w[0] ^ w[1]).count_ones() <= smudges
                && self[..i + 1]
                    .iter()
                    .rev()
                    .zip(self[i + 1..].iter())
                    .fold(0, |acc, (&a, &b)| acc + (a ^ b).count_ones())
                    == smudges
            {
                Some(i + 1)
            } else {
                None
            }
        })
    }
}

struct Mirror {
    points: Vec<Point>,
    extent: Point,
}

impl From<&str> for Mirror {
    fn from(chunk: &str) -> Self {
        let points = chunk
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
            .collect::<Vec<_>>();

        let extent = points.iter().fold(Point::new(0, 0), |acc, r| {
            Point::new(acc.x.max(r.x), acc.y.max(r.y))
        });

        Self { points, extent }
    }
}

impl Mirror {
    fn view(&self, axis: Axis) -> MirrorView {
        (0..=self.extent.view(axis))
            .map(|a| {
                (0..=self.extent.view(axis.inverse())).fold(0, |acc, b| {
                    acc | ((self.points.contains(&Point::new(a, b).pivoted(axis)) as usize) << b)
                })
            })
            .collect()
    }
}

struct Solution {
    mirrors: Vec<Mirror>,
}

impl Solution {
    fn solve(&self, smudges: u32) -> usize {
        self.mirrors
            .par_iter()
            .fold(
                || 0,
                |acc, mirror| {
                    acc + mirror
                        .view(Axis::H)
                        .find_reflection(smudges)
                        .map(|r| r * 100)
                        .or_else(|| mirror.view(Axis::V).find_reflection(smudges))
                        .unwrap_or(0)
                },
            )
            .sum()
    }
}

impl Solver for Solution {
    fn new(input: &str) -> Anyhow<Self> {
        Ok(Self {
            mirrors: input.split("\n\n").map(Mirror::from).collect(),
        })
    }

    fn part1(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self.solve(0))
    }

    fn part2(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self.solve(1))
    }
}

aoc::solution!();

#[cfg(test)]
mod test {
    use super::{Solution, Solver};

    const INPUT: &str = "#.##..##.
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
        let mut solution = Solution::new(INPUT).unwrap();
        let answer = solution.part1().unwrap().to_string();
        assert_eq!(answer, "405");
    }

    #[test]
    fn test_part2() {
        let mut solution = Solution::new(INPUT).unwrap();
        let answer = solution.part2().unwrap().to_string();
        assert_eq!(answer, "400");
    }
}
