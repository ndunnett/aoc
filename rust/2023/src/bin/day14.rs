use std::{collections::BTreeSet, ops::Add};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Direction {
    N,
    S,
    E,
    W,
}

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

#[derive(Debug, Clone)]
struct Solution {
    round_rocks: BTreeSet<Point>,
    square_rocks: BTreeSet<Point>,
    w: i32,
    h: i32,
}

impl Solution {
    fn tilted(&self, direction: Direction) -> Self {
        let move_p = match direction {
            Direction::N => Point::new(0, -1),
            Direction::S => Point::new(0, 1),
            Direction::E => Point::new(1, 0),
            Direction::W => Point::new(-1, 0),
        };

        let range_x = if direction == Direction::E {
            (0..self.w).rev().collect::<Vec<_>>()
        } else {
            (0..self.w).collect::<Vec<_>>()
        };

        let range_y = if direction == Direction::S {
            (0..self.h).rev().collect::<Vec<_>>()
        } else {
            (0..self.h).collect::<Vec<_>>()
        };

        let mut moved: BTreeSet<Point> = BTreeSet::new();

        for &y in range_y.iter() {
            for &x in range_x.iter() {
                let p = Point::new(x, y);

                if self.round_rocks.contains(&p) {
                    let mut new_p = p;

                    while range_x.contains(&(new_p + move_p).x)
                        && range_y.contains(&(new_p + move_p).y)
                        && !self.square_rocks.contains(&(new_p + move_p))
                        && !moved.contains(&(new_p + move_p))
                    {
                        new_p = new_p + move_p;
                    }

                    moved.insert(new_p);
                }
            }
        }

        Self {
            round_rocks: moved,
            square_rocks: self.square_rocks.clone(),
            w: self.w,
            h: self.h,
        }
    }

    fn cycled(&self, n: usize) -> Self {
        let mut platform = self.clone();
        let mut cache = HashMap::new();

        for i in 0..n {
            if let Some(j) = cache.get(&(platform.round_rocks)) {
                if (n - i) % (i - j) == 0 {
                    return platform;
                }
            }

            cache.insert(platform.round_rocks.clone(), i);

            for direction in [Direction::N, Direction::W, Direction::S, Direction::E] {
                platform = platform.tilted(direction);
            }
        }

        platform
    }

    fn weight(&self) -> i32 {
        self.round_rocks
            .iter()
            .fold(0, |acc, rock| acc + self.h - rock.y)
    }
}

impl Solver for Solution {
    fn new(input: &str) -> Anyhow<Self> {
        let mut round_rocks: BTreeSet<Point> = BTreeSet::new();
        let mut square_rocks: BTreeSet<Point> = BTreeSet::new();

        for (y, l) in input.lines().enumerate() {
            for (x, c) in l.chars().enumerate() {
                if c == 'O' {
                    round_rocks.insert(Point::new(x as i32, y as i32));
                } else if c == '#' {
                    square_rocks.insert(Point::new(x as i32, y as i32));
                }
            }
        }

        Ok(Self {
            round_rocks,
            square_rocks,
            w: input.lines().next().map_or(0, |s| s.len()) as i32,
            h: input.lines().count() as i32,
        })
    }

    fn part1(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self.tilted(Direction::N).weight())
    }

    fn part2(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self.cycled(1000000000).weight())
    }
}

aoc::solution!();

#[cfg(test)]
mod test {
    use super::{Solution, Solver};

    const INPUT: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn test_part1() {
        let mut solution = Solution::new(INPUT).unwrap();
        let answer = solution.part1().unwrap().to_string();
        assert_eq!(answer, "136");
    }

    #[test]
    fn test_part2() {
        let mut solution = Solution::new(INPUT).unwrap();
        let answer = solution.part2().unwrap().to_string();
        assert_eq!(answer, "64");
    }
}
