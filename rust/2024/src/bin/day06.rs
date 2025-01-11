#[derive(Clone, Copy, PartialEq)]
struct Point {
    x: u8,
    y: u8,
}

impl Point {
    fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    N,
    E,
    S,
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

    fn move_point(&self, p: &Point, size: u8) -> Option<Point> {
        match self {
            Self::N if p.y > 0 => Some(Point::new(p.x, p.y - 1)),
            Self::E if p.x < size - 1 => Some(Point::new(p.x + 1, p.y)),
            Self::S if p.y < size - 1 => Some(Point::new(p.x, p.y + 1)),
            Self::W if p.x > 0 => Some(Point::new(p.x - 1, p.y)),
            _ => None,
        }
    }
}

#[derive(Clone)]
struct Map {
    data: Vec<[u64; 3]>,
    size: u8,
}

impl Map {
    fn new(size: u8) -> Self {
        Self {
            data: vec![[0; 3]; size as usize],
            size,
        }
    }

    fn contains(&self, point: &Point) -> bool {
        let i = point.x % 64;
        (self.data[point.y as usize][(point.x - i) as usize / 64] >> i) & 1 > 0
    }

    fn insert(&mut self, point: Point) {
        let i = point.x % 64;
        self.data[point.y as usize][(point.x - i) as usize / 64] |= 1 << i;
    }
}

struct MapIterator<'a> {
    map: &'a Map,
    direction: Direction,
    position: Option<Point>,
}

impl Iterator for MapIterator<'_> {
    type Item = (Point, Direction);

    fn next(&mut self) -> Option<Self::Item> {
        let p = self.position?;

        if let Some(next) = self.direction.move_point(&p, self.map.size) {
            if self.map.contains(&next) {
                self.direction = self.direction.turn();
            } else {
                self.position = Some(next);
            }
        } else {
            self.position = None;
        }

        Some((p, self.direction))
    }
}

#[derive(Clone)]
struct Solution {
    map: Map,
    start: Point,
}

impl Solution {
    fn iter(&self) -> MapIterator<'_> {
        MapIterator {
            map: &self.map,
            direction: Direction::N,
            position: Some(self.start),
        }
    }
}

impl Solver for Solution {
    fn new(input: &str) -> Anyhow<Self> {
        let mut map = Map::new(input.lines().count() as u8);
        let mut start = None;
        let mut x = 0_u8;
        let mut y = 0_u8;

        for b in input.bytes() {
            match b {
                b'#' => {
                    map.insert(Point::new(x, y));
                    x += 1;
                }
                b'^' => {
                    start = Some(Point::new(x, y));
                    x += 1;
                }
                b'\n' => {
                    x = 0;
                    y += 1;
                }
                _ => x += 1,
            }
        }

        let start = start.ok_or(anyhow!("failed to find start"))?;
        Ok(Self { map, start })
    }

    fn part1(&mut self) -> Anyhow<impl fmt::Display> {
        let mut seen = Map::new(self.map.size);

        Ok(self
            .iter()
            .filter(|(point, _)| {
                if seen.contains(point) {
                    false
                } else {
                    seen.insert(*point);
                    true
                }
            })
            .count())
    }

    fn part2(&mut self) -> Anyhow<impl fmt::Display> {
        let mut seen = Map::new(self.map.size);

        let candidates = self
            .iter()
            .skip(1)
            .filter_map(|(point, _)| {
                if seen.contains(&point) {
                    None
                } else {
                    seen.insert(point);
                    Some(point)
                }
            })
            .collect::<Vec<_>>();

        Ok(candidates
            .par_iter()
            .filter(|&&point| {
                let mut new = self.clone();
                new.map.insert(point);

                let mut seen: [_; 4] = std::array::from_fn(|_| Map::new(self.map.size));

                for ((ap, ad), (_, bd)) in new.iter().tuple_windows() {
                    if ad != bd {
                        if seen[ad as usize].contains(&ap) {
                            return true;
                        } else {
                            seen[ad as usize].insert(ap);
                        }
                    }
                }

                false
            })
            .count())
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
