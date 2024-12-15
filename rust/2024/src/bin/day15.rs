#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Clone)]
enum Direction {
    N,
    E,
    S,
    W,
}

impl TryFrom<u8> for Direction {
    type Error = Error;

    fn try_from(b: u8) -> Result<Self, Self::Error> {
        match b {
            b'^' => Ok(Self::N),
            b'>' => Ok(Self::E),
            b'v' => Ok(Self::S),
            b'<' => Ok(Self::W),
            _ => Err(anyhow!("failed to parse direction: {:?}", b as char)),
        }
    }
}

impl Direction {
    fn move_point(&self, from: &Point) -> Point {
        match self {
            Self::N => Point::new(from.x, from.y - 1),
            Self::E => Point::new(from.x + 1, from.y),
            Self::S => Point::new(from.x, from.y + 1),
            Self::W => Point::new(from.x - 1, from.y),
        }
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
enum Tile {
    Wall,
    Box,
    BoxLeft,
    BoxRight,
}

#[derive(Clone)]
struct Solution {
    map: FxHashMap<Point, Tile>,
    robot: Point,
    moves: Vec<Direction>,
}

impl Solution {
    fn solve(&mut self) -> i32 {
        'outer: for direction in &self.moves {
            let robot_next = direction.move_point(&self.robot);

            if !self.map.contains_key(&robot_next) {
                self.robot = robot_next;
                continue;
            }

            let mut queue = vec![robot_next];
            let mut moves = Vec::new();

            while let Some(p) = queue.pop() {
                match (direction, self.map.get(&p)) {
                    (_, Some(Tile::Wall)) => {
                        continue 'outer;
                    }
                    (Direction::N | Direction::S, Some(Tile::BoxLeft)) => {
                        let right = Direction::E.move_point(&p);
                        queue.push(direction.move_point(&p));
                        queue.push(direction.move_point(&right));
                        moves.push(p);
                        moves.push(right);
                    }
                    (Direction::N | Direction::S, Some(Tile::BoxRight)) => {
                        let left = Direction::W.move_point(&p);
                        queue.push(direction.move_point(&p));
                        queue.push(direction.move_point(&left));
                        moves.push(p);
                        moves.push(left);
                    }
                    (Direction::E | Direction::W, Some(Tile::BoxLeft | Tile::BoxRight)) => {
                        let front = direction.move_point(&p);
                        queue.push(direction.move_point(&front));
                        moves.push(p);
                        moves.push(front);
                    }
                    (_, Some(Tile::Box)) => {
                        queue.push(direction.move_point(&p));
                        moves.push(p);
                    }
                    _ => {}
                }
            }

            if !moves.is_empty() {
                let moved = moves
                    .iter()
                    .filter_map(|p| Some((direction.move_point(p), self.map.remove(p)?)))
                    .collect::<Vec<_>>();

                for (next, tile) in moved {
                    self.map.insert(next, tile);
                }
            }

            if !self.map.contains_key(&robot_next) {
                self.robot = robot_next;
            }
        }

        self.map
            .iter()
            .filter_map(|(p, &tile)| {
                if tile == Tile::Box || tile == Tile::BoxLeft {
                    Some(p)
                } else {
                    None
                }
            })
            .fold(0, |acc, p| acc + p.x + p.y * 100)
    }

    fn expanded(&self) -> Self {
        Self {
            map: self
                .map
                .par_iter()
                .flat_map(|(p, tile)| match tile {
                    Tile::Wall => {
                        vec![
                            (Point::new(p.x * 2, p.y), Tile::Wall),
                            (Point::new(p.x * 2 + 1, p.y), Tile::Wall),
                        ]
                    }
                    Tile::Box => {
                        vec![
                            (Point::new(p.x * 2, p.y), Tile::BoxLeft),
                            (Point::new(p.x * 2 + 1, p.y), Tile::BoxRight),
                        ]
                    }
                    _ => unreachable!(),
                })
                .collect(),
            robot: Point::new(self.robot.x * 2, self.robot.y),
            moves: self.moves.clone(),
        }
    }
}

impl Solver for Solution {
    fn new(input: &str) -> Anyhow<Self> {
        let (map_chunk, moves_chunk) = input
            .split_once("\n\n")
            .ok_or(anyhow!("failed to split input"))?;

        let mut map = FxHashMap::default();
        let mut robot = None;

        for (line, y) in map_chunk.lines().zip(0..i32::MAX) {
            for (b, x) in line.bytes().zip(0..i32::MAX) {
                match b {
                    b'#' => {
                        map.insert(Point::new(x, y), Tile::Wall);
                    }
                    b'O' => {
                        map.insert(Point::new(x, y), Tile::Box);
                    }
                    b'@' => robot = Some(Point::new(x, y)),
                    _ => {}
                }
            }
        }

        let robot = robot.ok_or(anyhow!("failed to find robot"))?;

        let moves = moves_chunk
            .bytes()
            .filter(|&b| b != b'\n')
            .map(Direction::try_from)
            .collect::<Anyhow<Vec<_>>>()?;

        Ok(Self { map, robot, moves })
    }

    fn part1(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self.clone().solve())
    }

    fn part2(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self.clone().expanded().solve())
    }
}

aoc::solution!();

#[cfg(test)]
mod test {
    use super::{Solution, Solver};

    const INPUT_SMALL_1: &str = r"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
";

    const INPUT_SMALL_2: &str = r"#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^
";

    const INPUT_LARGE: &str = r"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########


<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
";

    #[test]
    fn test_part1_small() {
        let mut solution = Solution::new(INPUT_SMALL_1).unwrap();
        let answer = solution.part1().unwrap().to_string();
        assert_eq!(answer, "2028");
    }

    #[test]
    fn test_part1_large() {
        let mut solution = Solution::new(INPUT_LARGE).unwrap();
        let answer = solution.part1().unwrap().to_string();
        assert_eq!(answer, "10092");
    }

    #[test]
    fn test_part2_small() {
        let mut solution = Solution::new(INPUT_SMALL_2).unwrap();
        let answer = solution.part2().unwrap().to_string();
        assert_eq!(answer, "618");
    }

    #[test]
    fn test_part2_large() {
        let mut solution = Solution::new(INPUT_LARGE).unwrap();
        let answer = solution.part2().unwrap().to_string();
        assert_eq!(answer, "9021");
    }
}
