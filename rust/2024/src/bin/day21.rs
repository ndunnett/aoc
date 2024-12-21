#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    x: u8,
    y: u8,
}

impl Point {
    const fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }

    fn distance(&self, other: &Point) -> u8 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    N,
    E,
    S,
    W,
}

impl Direction {
    fn next_move(&self, p: Point, map: &FxHashMap<Point, u8>) -> Option<Point> {
        let next_p = match self {
            Self::N if p.y > 0 => Some(Point::new(p.x, p.y - 1)),
            Self::E => Some(Point::new(p.x + 1, p.y)),
            Self::S => Some(Point::new(p.x, p.y + 1)),
            Self::W if p.x > 0 => Some(Point::new(p.x - 1, p.y)),
            _ => None,
        };

        if map.contains_key(&next_p?) {
            next_p
        } else {
            None
        }
    }
}

impl From<&Direction> for u8 {
    fn from(d: &Direction) -> Self {
        match d {
            Direction::N => b'^',
            Direction::E => b'>',
            Direction::S => b'v',
            Direction::W => b'<',
        }
    }
}

struct Keypad {
    map: FxHashMap<Point, u8>,
    positions: FxHashMap<u8, Point>,
}

impl Keypad {
    fn new<I>(iter: I) -> Self
    where
        I: Iterator<Item = (Point, u8)> + Clone,
        FxHashMap<Point, u8>: FromIterator<(Point, u8)>,
        FxHashMap<u8, Point>: FromIterator<(u8, Point)>,
    {
        Self {
            map: iter.clone().collect(),
            positions: iter.map(|(k, v)| (v, k)).collect(),
        }
    }

    const DOOR_KEYS: [(Point, u8); 11] = [
        (Point::new(0, 0), b'7'), //     0   1   2
        (Point::new(1, 0), b'8'), //   +---+---+---+
        (Point::new(2, 0), b'9'), // 0 | 7 | 8 | 9 |
        (Point::new(0, 1), b'4'), //   +---+---+---+
        (Point::new(1, 1), b'5'), // 1 | 4 | 5 | 6 |
        (Point::new(2, 1), b'6'), //   +---+---+---+
        (Point::new(0, 2), b'1'), // 2 | 1 | 2 | 3 |
        (Point::new(1, 2), b'2'), //   +---+---+---+
        (Point::new(2, 2), b'3'), // 3     | 0 | A |
        (Point::new(1, 3), b'0'), //       +---+---+
        (Point::new(2, 3), b'A'),
    ];

    const ROBOT_KEYS: [(Point, u8); 5] = [
        (Point::new(1, 0), b'^'), //     0   1   2
        (Point::new(2, 0), b'A'), //       +---+---+
        (Point::new(0, 1), b'<'), // 0     | ^ | A |
        (Point::new(1, 1), b'v'), //   +---+---+---+
        (Point::new(2, 1), b'>'), // 1 | < | v | > |
                                  //   +---+---+---+
    ];
}

struct Solution {
    door: Keypad,
    robot: Keypad,
    targets: Vec<(usize, Vec<u8>)>,
}

type Cache = FxHashMap<(u8, u8, usize), usize>;

impl Solution {
    fn traverse(&self, keypad: &Keypad, a: u8, b: u8, depth: usize, cache: &mut Cache) -> usize {
        if let Some(&result) = cache.get(&(a, b, depth)) {
            return result;
        }

        let from = keypad.positions[&a];
        let to = keypad.positions[&b];

        if depth == 0 {
            return to.distance(&from) as usize + 1;
        }

        let mut moves = Vec::new();

        if from.x < to.x {
            moves.extend([Direction::E].repeat((to.x - from.x) as usize));
        } else {
            moves.extend([Direction::W].repeat((from.x - to.x) as usize));
        }

        if from.y < to.y {
            moves.extend([Direction::S].repeat((to.y - from.y) as usize));
        } else {
            moves.extend([Direction::N].repeat((from.y - to.y) as usize));
        }

        let result = moves
            .iter()
            .permutations(moves.len())
            .filter_map(|moves| {
                let mut p = from;

                for &d in &moves {
                    p = d.next_move(p, &keypad.map)?;
                }

                Some(
                    [b'A']
                        .into_iter()
                        .chain(moves.into_iter().map(u8::from))
                        .chain([b'A'])
                        .tuple_windows()
                        .map(|(a, b)| self.traverse(&self.robot, a, b, depth - 1, cache))
                        .sum::<usize>(),
                )
            })
            .min()
            .expect("failed to find move set");

        cache.insert((a, b, depth), result);
        result
    }

    fn solve(&self, depth: usize) -> Anyhow<usize> {
        let mut cache = Cache::default();

        Ok(self
            .targets
            .iter()
            .map(|(number, seq)| {
                number
                    * [b'A']
                        .iter()
                        .chain(seq.iter())
                        .tuple_windows()
                        .map(|(&a, &b)| self.traverse(&self.door, a, b, depth, &mut cache))
                        .sum::<usize>()
            })
            .sum::<usize>())
    }
}

impl Solver for Solution {
    fn new(input: &str) -> Anyhow<Self> {
        Ok(Self {
            door: Keypad::new(Keypad::DOOR_KEYS.into_iter()),
            robot: Keypad::new(Keypad::ROBOT_KEYS.into_iter()),
            targets: input
                .lines()
                .map(|line| {
                    Ok((
                        line.strip_suffix('A')
                            .ok_or(anyhow!("failed to strip letter"))?
                            .parse::<usize>()?,
                        line.bytes().collect::<Vec<_>>(),
                    ))
                })
                .collect::<Anyhow<Vec<_>>>()?,
        })
    }

    fn part1(&mut self) -> Anyhow<impl fmt::Display> {
        self.solve(2)
    }

    fn part2(&mut self) -> Anyhow<impl fmt::Display> {
        self.solve(25)
    }
}

aoc::solution!();

#[cfg(test)]
mod test {
    use super::{Solution, Solver};

    const INPUT: &str = r"029A
980A
179A
456A
379A";

    #[test]
    fn test_part1() {
        let mut solution = Solution::new(INPUT).unwrap();
        let answer = solution.part1().unwrap().to_string();
        assert_eq!(answer, "126384");
    }

    #[test]
    fn test_part2() {
        let mut solution = Solution::new(INPUT).unwrap();
        let answer = solution.part2().unwrap().to_string();
        assert_eq!(answer, "154115708116294");
    }
}
