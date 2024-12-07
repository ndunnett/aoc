use std::iter::Peekable;

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct Point {
    x: u8,
    y: u8,
}

impl Point {
    fn new<Int>(x: Int, y: Int) -> Self
    where
        u8: TryFrom<Int>,
        <u8 as std::convert::TryFrom<Int>>::Error: fmt::Debug,
    {
        Self {
            x: x.try_into().expect("failed to create point"),
            y: y.try_into().expect("failed to create point"),
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
enum Direction {
    N,
    S,
    E,
    W,
}

impl Direction {
    fn turn(&self) -> Self {
        match self {
            Self::N => Self::E,
            Self::E => Self::S,
            Self::S => Self::W,
            Self::W => Self::N,
        }
    }
}

#[derive(Clone)]
struct Map {
    obstacles: FxHashSet<Point>,
    start: Point,
    size: u8,
}

impl TryFrom<&str> for Map {
    type Error = Error;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let mut obstacles = FxHashSet::default();
        let mut start = None;

        for (y, line) in input.lines().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if ch == '#' {
                    obstacles.insert(Point::new(x, y));
                } else if ch == '^' {
                    start = Some(Point::new(x, y));
                }
            }
        }

        Ok(Self {
            obstacles,
            start: start.ok_or(anyhow!("failed to find start"))?,
            size: input.lines().count() as u8,
        })
    }
}

type Index = Box<dyn Iterator<Item = (Point, Direction)>>;

impl Map {
    fn iter(&self) -> MapIterator {
        MapIterator::from(self)
    }

    fn index(&self, s: Point, dir: Direction) -> Index {
        match dir {
            Direction::N => Box::new((0..s.y + 1).rev().map(move |y| (Point::new(s.x, y), dir))),
            Direction::E => Box::new((s.x..self.size).map(move |x| (Point::new(x, s.y), dir))),
            Direction::S => Box::new((s.y..self.size).map(move |y| (Point::new(s.x, y), dir))),
            Direction::W => Box::new((0..s.x + 1).rev().map(move |x| (Point::new(x, s.y), dir))),
        }
    }
}

struct MapIterator<'a> {
    map: &'a Map,
    index: Peekable<Index>,
}

impl<'a> From<&'a Map> for MapIterator<'a> {
    fn from(map: &'a Map) -> Self {
        Self {
            map,
            index: map.index(map.start, Direction::N).peekable(),
        }
    }
}

impl Iterator for MapIterator<'_> {
    type Item = (Point, Direction);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((point, direction)) = self.index.next() {
            if let Some((peeked, _)) = self.index.peek() {
                if self.map.obstacles.contains(peeked) {
                    let turned = direction.turn();
                    self.index = self.map.index(point, turned).peekable();
                    Some((point, turned))
                } else {
                    Some((point, direction))
                }
            } else {
                Some((point, direction))
            }
        } else {
            None
        }
    }
}

struct Solution {
    map: Map,
}

impl Solver for Solution {
    fn new(input: &str) -> Anyhow<Self> {
        Ok(Self {
            map: Map::try_from(input)?,
        })
    }

    fn part1(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self
            .map
            .iter()
            .map(|(point, _)| point)
            .collect::<FxHashSet<Point>>()
            .len())
    }

    fn part2(&mut self) -> Anyhow<impl fmt::Display> {
        let points = self
            .map
            .iter()
            .map(|(point, _)| point)
            .collect::<FxHashSet<Point>>();

        Ok(points
            .par_iter()
            .fold(
                || 0,
                |mut acc, &point| {
                    if point == self.map.start {
                        return acc;
                    }

                    let mut new_map = self.map.clone();
                    new_map.obstacles.insert(point);

                    let chunks = new_map.iter().chunk_by(|(direction, _)| *direction);
                    let mut path = FxHashSet::default();

                    for (_, chunk) in chunks.into_iter() {
                        let next = chunk.last().expect("failed to chunk path");

                        if !path.insert(next) {
                            acc += 1;
                            break;
                        }
                    }

                    acc
                },
            )
            .sum::<usize>())
    }
}

aoc::solution!();

#[cfg(test)]
mod test {
    use super::{Solution, Solver};

    const INPUT: &str = r"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_part1() {
        let mut solution = Solution::new(INPUT).unwrap();
        let answer = solution.part1().unwrap().to_string();
        assert_eq!(answer, "41");
    }

    #[test]
    fn test_part2() {
        let mut solution = Solution::new(INPUT).unwrap();
        let answer = solution.part2().unwrap().to_string();
        assert_eq!(answer, "6");
    }
}
