#[derive(Clone, Copy, Eq, PartialEq, Hash)]
enum Direction {
    N,
    S,
    E,
    W,
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn in_bounds<T>(&self, vec: &[Vec<T>]) -> bool {
        self.y < vec.len() && self.x < vec[0].len()
    }

    fn move_towards(&mut self, direction: Direction) -> bool {
        match direction {
            Direction::N => {
                if self.y == 0 {
                    return false;
                } else {
                    self.y -= 1;
                }
            }
            Direction::W => {
                if self.x == 0 {
                    return false;
                } else {
                    self.x -= 1;
                }
            }
            Direction::S => self.y += 1,
            Direction::E => self.x += 1,
        }

        true
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Tile {
    Empty,
    ForwardSlash,
    BackSlash,
    Horizontal,
    Vertical,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '/' => Self::ForwardSlash,
            '\\' => Self::BackSlash,
            '-' => Self::Horizontal,
            '|' => Self::Vertical,
            _ => Self::Empty,
        }
    }
}

struct State {
    energised: FxHashMap<Point, FxHashSet<Direction>>,
}

impl State {
    fn new() -> Self {
        Self {
            energised: FxHashMap::default(),
        }
    }

    fn energise(&mut self, position: Point, direction: Direction) -> bool {
        if let Some(state) = self.energised.get_mut(&position) {
            state.insert(direction)
        } else {
            self.energised
                .insert(position, FxHashSet::from_iter([direction]))
                .is_none()
        }
    }

    fn finalise(self) -> Vec<Point> {
        self.energised.into_keys().collect()
    }
}

#[derive(Clone)]
struct Solution {
    contraption: Vec<Vec<Tile>>,
}

impl Solution {
    fn get(&self, position: Point) -> Tile {
        self.contraption[position.y][position.x]
    }

    fn trace_beam(&self, position: Point, direction: Direction) -> Vec<Point> {
        let mut queue = vec![(position, direction)];
        let mut state = State::new();

        while let Some((mut pos, mut dir)) = queue.pop() {
            while pos.in_bounds(&self.contraption) && state.energise(pos, dir) {
                match self.get(pos) {
                    Tile::Vertical => {
                        if dir == Direction::E || dir == Direction::W {
                            queue.push((pos, Direction::N));
                            queue.push((pos, Direction::S));
                            break;
                        }
                    }
                    Tile::Horizontal => {
                        if dir == Direction::N || dir == Direction::S {
                            queue.push((pos, Direction::E));
                            queue.push((pos, Direction::W));
                            break;
                        }
                    }
                    Tile::ForwardSlash => {
                        dir = match dir {
                            Direction::N => Direction::E,
                            Direction::S => Direction::W,
                            Direction::E => Direction::N,
                            Direction::W => Direction::S,
                        };
                    }
                    Tile::BackSlash => {
                        dir = match dir {
                            Direction::N => Direction::W,
                            Direction::S => Direction::E,
                            Direction::E => Direction::S,
                            Direction::W => Direction::N,
                        };
                    }
                    Tile::Empty => {}
                }

                if !pos.move_towards(dir) {
                    break;
                }
            }
        }

        state.finalise()
    }
}

impl Solver for Solution {
    fn new(input: &str) -> Anyhow<Self> {
        Ok(Self {
            contraption: input
                .lines()
                .map(|line| line.chars().map(Tile::from).collect::<Vec<_>>())
                .collect::<Vec<_>>(),
        })
    }

    fn part1(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self.trace_beam(Point::new(0, 0), Direction::E).len())
    }

    fn part2(&mut self) -> Anyhow<impl fmt::Display> {
        let extent = Point::new(self.contraption[0].len() - 1, self.contraption.len() - 1);

        Ok((0..extent.x)
            .map(|x| (Point::new(x, 0), Direction::S))
            .chain((0..=extent.x).map(|x| (Point::new(x, extent.y), Direction::N)))
            .chain((0..=extent.y).map(|y| (Point::new(0, y), Direction::E)))
            .chain((0..=extent.y).map(|y| (Point::new(extent.x, y), Direction::W)))
            .par_bridge()
            .map(|(pos, dir)| self.trace_beam(pos, dir).len())
            .max()
            .unwrap_or(0))
    }
}

aoc::solution!();

#[cfg(test)]
mod test {
    use super::{Solution, Solver};

    const INPUT: &str = r".|...\....
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
        let mut solution = Solution::new(INPUT).unwrap();
        let answer = solution.part1().unwrap().to_string();
        assert_eq!(answer, "46");
    }

    #[test]
    fn test_part2() {
        let mut solution = Solution::new(INPUT).unwrap();
        let answer = solution.part2().unwrap().to_string();
        assert_eq!(answer, "51");
    }
}
