use std::ops::Add;

use itertools::Itertools;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point::new(self.x + other.x, self.y + other.y)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
enum Pipe {
    Cross,
    Vertical,
    Horizontal,
    BendNE,
    BendNW,
    BendSW,
    BendSE,
}

impl Pipe {
    fn from_char(c: char) -> Option<Pipe> {
        match c {
            'S' => Some(Pipe::Cross),
            '|' => Some(Pipe::Vertical),
            '-' => Some(Pipe::Horizontal),
            'L' => Some(Pipe::BendNE),
            'J' => Some(Pipe::BendNW),
            '7' => Some(Pipe::BendSW),
            'F' => Some(Pipe::BendSE),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Tile {
    point: Point,
    pipe: Pipe,
}

impl Tile {
    fn new(point: Point, pipe: Pipe) -> Tile {
        Tile { point, pipe }
    }
}

pub struct Solution {
    maze: HashMap<Point, Pipe>,
}

impl Solution {
    fn neighbours(&self, tile: &Tile) -> Vec<Tile> {
        match tile.pipe {
            Pipe::Vertical => vec![Point::new(0, -1), Point::new(0, 1)],
            Pipe::BendNE => vec![Point::new(0, -1), Point::new(1, 0)],
            Pipe::BendNW => vec![Point::new(0, -1), Point::new(-1, 0)],
            Pipe::BendSE => vec![Point::new(0, 1), Point::new(1, 0)],
            Pipe::BendSW => vec![Point::new(0, 1), Point::new(-1, 0)],
            Pipe::Horizontal => vec![Point::new(1, 0), Point::new(-1, 0)],
            Pipe::Cross => vec![
                Point::new(-1, 0),
                Point::new(1, 0),
                Point::new(0, 1),
                Point::new(0, -1),
            ],
        }
        .iter()
        .filter_map(|&offset| {
            let p = tile.point + offset;

            if let Some(&t) = self.maze.get(&p) {
                Some(Tile::new(p, t))
            } else {
                None
            }
        })
        .collect()
    }

    fn connections(&self, tile: &Tile) -> Vec<Tile> {
        self.neighbours(tile)
            .into_iter()
            .filter(|n| self.neighbours(n).contains(tile))
            .collect_vec()
    }

    fn find_circuit(&self, from: &Tile, seen: &[Tile]) -> Option<Vec<Tile>> {
        for c in self.connections(from) {
            if c.pipe == Pipe::Cross {
                return Some(seen.to_vec());
            } else if !seen.contains(&c) {
                return self.find_circuit(&c, &[seen, &[c]].concat());
            }
        }

        None
    }

    fn find_start(&self) -> Tile {
        self.maze
            .iter()
            .find_map(|(&point, &pipe)| {
                if pipe == Pipe::Cross {
                    Some(Tile::new(point, pipe))
                } else {
                    None
                }
            })
            .unwrap()
    }

    fn extent(&self) -> (Point, Point) {
        let (min_x, max_x, min_y, max_y) =
            self.maze
                .iter()
                .fold((0, 0, 0, 0), |(min_x, max_x, min_y, max_y), (point, _)| {
                    (
                        min_x.min(point.x),
                        max_x.max(point.x),
                        min_y.min(point.y),
                        max_y.max(point.y),
                    )
                });

        (Point::new(min_x, min_y), Point::new(max_x, max_y))
    }
}

impl Solver for Solution {
    fn new(input: &str) -> Anyhow<Self> {
        let maze = input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(|(x, c)| {
                        Pipe::from_char(c).map(|j| Tile::new(Point::new(x as i32, y as i32), j))
                    })
                    .collect_vec()
            })
            .map(|p| (p.point, p.pipe))
            .collect();

        let mut sol = Self { maze };
        let start = sol.find_start();

        if let Some(circuit) = sol.find_circuit(&start, &[start]) {
            sol.maze = circuit.iter().map(|p| (p.point, p.pipe)).collect();
        }

        Ok(sol)
    }

    fn part1(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self.maze.len() / 2)
    }

    fn part2(&mut self) -> Anyhow<impl fmt::Display> {
        let (min, max) = self.extent();

        Ok((min.y..=max.y)
            .map(|y| {
                (min.x..=max.x)
                    .fold((0, false), |(acc, in_loop), x| {
                        match (in_loop, self.maze.get(&Point::new(x, y))) {
                            (_, Some(Pipe::Vertical | Pipe::BendNW | Pipe::BendNE)) => {
                                (acc, !in_loop)
                            }
                            (true, None) => (acc + 1, in_loop),
                            _ => (acc, in_loop),
                        }
                    })
                    .0
            })
            .sum::<i32>())
    }
}

aoc::solution!();

#[cfg(test)]
mod test {
    use super::{Solution, Solver};

    const INPUT1: &str = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";

    const INPUT2: &str = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

    const INPUT3: &str = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

    const INPUT4: &str = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";

    const INPUT5: &str = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

    #[test]
    fn test_part1_1() {
        let mut solution = Solution::new(INPUT1).unwrap();
        let answer = solution.part1().unwrap().to_string();
        assert_eq!(answer, "4");
    }

    #[test]
    fn test_part1_2() {
        let mut solution = Solution::new(INPUT2).unwrap();
        let answer = solution.part1().unwrap().to_string();
        assert_eq!(answer, "8");
    }

    #[test]
    fn test_part2_1() {
        let mut solution = Solution::new(INPUT3).unwrap();
        let answer = solution.part2().unwrap().to_string();
        assert_eq!(answer, "4");
    }

    #[test]
    fn test_part2_2() {
        let mut solution = Solution::new(INPUT4).unwrap();
        let answer = solution.part2().unwrap().to_string();
        assert_eq!(answer, "8");
    }

    #[test]
    fn test_part2_3() {
        let mut solution = Solution::new(INPUT5).unwrap();
        let answer = solution.part2().unwrap().to_string();
        assert_eq!(answer, "10");
    }
}
