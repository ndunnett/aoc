aoc::solution!();

use std::{
    collections::{HashMap, HashSet},
    ops::Add,
};

use rayon::prelude::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Direction {
    N,
    S,
    E,
    W,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
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

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Tile {
    Empty,
    ForwardSlash,
    BackSlash,
    Horizontal,
    Vertical,
}

impl Tile {
    fn from_char(c: char) -> Tile {
        match c {
            '/' => Tile::ForwardSlash,
            '\\' => Tile::BackSlash,
            '-' => Tile::Horizontal,
            '|' => Tile::Vertical,
            _ => Tile::Empty,
        }
    }
}

type Contraption = HashMap<Point, Tile>;

trait Solution {
    fn parse(input: &str) -> Contraption;
    fn trace_beam(&self, position: Point, direction: Direction) -> Vec<Point>;
}

impl Solution for Contraption {
    fn parse(input: &str) -> Contraption {
        input
            .lines()
            .enumerate()
            .flat_map(|(y, l)| {
                l.chars()
                    .enumerate()
                    .map(move |(x, c)| (Point::new(x as i32, y as i32), Tile::from_char(c)))
            })
            .collect()
    }

    fn trace_beam(&self, position: Point, direction: Direction) -> Vec<Point> {
        let mut beams = vec![(position, direction)];
        let mut energised: HashMap<Point, HashSet<Direction>> = HashMap::new();

        let mut energise_tile = |p: Point, d: Direction| {
            if self.contains_key(&p) {
                if let Some(e) = energised.get_mut(&p) {
                    e.insert(d)
                } else {
                    energised.insert(p, HashSet::from_iter([d])).is_none()
                }
            } else {
                false
            }
        };

        while let Some((mut p, mut d)) = beams.pop() {
            while energise_tile(p, d) {
                if let Some(tile) = self.get(&p) {
                    match tile {
                        Tile::Vertical => {
                            if d == Direction::E || d == Direction::W {
                                beams.push((p, Direction::N));
                                beams.push((p, Direction::S));
                                break;
                            }
                        }
                        Tile::Horizontal => {
                            if d == Direction::N || d == Direction::S {
                                beams.push((p, Direction::E));
                                beams.push((p, Direction::W));
                                break;
                            }
                        }
                        Tile::ForwardSlash => {
                            d = match d {
                                Direction::N => Direction::E,
                                Direction::S => Direction::W,
                                Direction::E => Direction::N,
                                Direction::W => Direction::S,
                            };
                        }
                        Tile::BackSlash => {
                            d = match d {
                                Direction::N => Direction::W,
                                Direction::S => Direction::E,
                                Direction::E => Direction::S,
                                Direction::W => Direction::N,
                            };
                        }
                        Tile::Empty => {}
                    }

                    p = p + match d {
                        Direction::N => Point::new(0, -1),
                        Direction::S => Point::new(0, 1),
                        Direction::E => Point::new(1, 0),
                        Direction::W => Point::new(-1, 0),
                    };
                } else {
                    break;
                }
            }
        }

        energised.into_keys().collect()
    }
}

fn part1(input: &str) -> usize {
    Contraption::parse(input)
        .trace_beam(Point::new(0, 0), Direction::E)
        .len()
}

fn part2(input: &str) -> usize {
    let c = Contraption::parse(input);
    let extent = c.keys().fold(Point { x: 0, y: 0 }, |acc, p| Point {
        x: acc.x.max(p.x),
        y: acc.y.max(p.y),
    });

    (0..=extent.x)
        .map(|x| (Point::new(x, 0), Direction::S))
        .chain((0..=extent.x).map(|x| (Point::new(x, extent.y), Direction::N)))
        .chain((0..=extent.y).map(|y| (Point::new(0, y), Direction::E)))
        .chain((0..=extent.y).map(|y| (Point::new(extent.x, y), Direction::W)))
        .par_bridge()
        .map(|(p, d)| c.trace_beam(p, d).len())
        .max()
        .unwrap_or(0)
}

#[cfg(test)]
mod test {
    use super::{part1, part2};

    const INPUT1: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT1), 46);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT1), 51);
    }
}
