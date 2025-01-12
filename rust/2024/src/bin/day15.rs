#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    x: u8,
    y: u8,
}

impl Point {
    fn new(x: u8, y: u8) -> Self {
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
    Empty,
    Wall,
    Box,
    BoxLeft,
    BoxRight,
}

#[derive(Clone)]
struct Map {
    data: [[Tile; 100]; 50],
}

impl Map {
    fn new() -> Self {
        Self {
            data: [[Tile::Empty; 100]; 50],
        }
    }

    fn get(&self, p: &Point) -> Tile {
        self.data[p.y as usize][p.x as usize]
    }

    fn insert(&mut self, p: &Point, t: Tile) {
        self.data[p.y as usize][p.x as usize] = t;
    }
}

#[derive(Clone)]
struct Solution {
    map: Map,
    start: Point,
    moves: Vec<Direction>,
}

impl Solution {
    fn solve(&mut self) -> usize {
        let mut position = self.start;

        self.moves.iter().for_each(|direction| {
            let next_position = direction.move_point(&position);

            // if the next tile is empty, we can move without consideration
            if self.map.get(&next_position) == Tile::Empty {
                position = next_position;
                return;
            }

            let mut pushes = Vec::with_capacity(32);
            let mut queue = Vec::with_capacity(10);
            queue.push(next_position);

            // macro to conveniently add the next position to queue and push the current position to the vec of moves
            macro_rules! queue {
                ($($p:expr),+) => {$({
                    let next = direction.move_point(&$p);
                    queue.push(next);
                    pushes.push($p);
                })+};
            }

            // use DFS to find all boxes that will be pushed on the given move
            while let Some(p) = queue.pop() {
                match (direction, self.map.get(&p)) {
                    // if we reach a wall, the move isn't possible
                    (_, Tile::Wall) => return,
                    // when pushing north/south and we reach a big box, we need to push both sides
                    (Direction::N | Direction::S, Tile::BoxLeft) => {
                        // queue up next moves for the current position (left side of box) and adjacent position to the east (right side of box)
                        queue!(p, Direction::E.move_point(&p));
                    }
                    (Direction::N | Direction::S, Tile::BoxRight) => {
                        // queue up next moves for the current position (right side of box) and adjacent position to the west (left side of box)
                        queue!(p, Direction::W.move_point(&p));
                    }
                    // all other pushes can be handled individually
                    (_, Tile::Box | Tile::BoxLeft | Tile::BoxRight) => {
                        queue!(p);
                    }
                    _ => {}
                }
            }

            // pop all tiles to be pushed from the map and collect them with new positions
            let pushed = pushes
                .iter()
                .filter_map(|from| {
                    let tile = self.map.get(from);

                    if tile != Tile::Empty {
                        self.map.insert(from, Tile::Empty);
                        Some((direction.move_point(from), tile))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            // insert collected tiles back into the map in the new position
            for (to, tile) in pushed {
                self.map.insert(&to, tile);
            }

            // finally move the robot
            position = next_position;
        });

        self.map
            .data
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter().enumerate().filter_map(move |(x, &tile)| {
                    if tile == Tile::Box || tile == Tile::BoxLeft {
                        Some(x + y * 100)
                    } else {
                        None
                    }
                })
            })
            .sum()
    }

    fn expanded(&self) -> Self {
        let mut new = self.clone();

        for row in new.map.data.iter_mut() {
            for (x, tile) in (*row)
                .into_iter()
                .flat_map(|tile| match tile {
                    // double the width of walls and empty tiles and transform boxes into big boxes:
                    // '.' => '..'
                    // '#' => '##'
                    // 'O' => '[]'
                    Tile::Empty => [Tile::Empty; 2].into_iter(),
                    Tile::Wall => [Tile::Wall; 2].into_iter(),
                    Tile::Box => [Tile::BoxLeft, Tile::BoxRight].into_iter(),
                    _ => unreachable!(),
                })
                .take(100)
                .enumerate()
            {
                row[x] = tile;
            }
        }

        // starting position is twice as wide due to expansion
        new.start.x *= 2;
        new
    }
}

impl Solver for Solution {
    fn new(input: &str) -> Anyhow<Self> {
        let (map_chunk, moves_chunk) = input
            .split_once("\n\n")
            .ok_or(anyhow!("failed to split input"))?;

        let mut map = Map::new();
        let mut start = None;
        let mut p = Point::new(0, 0);

        for b in map_chunk.bytes() {
            match b {
                b'\n' => {
                    p.x = 0;
                    p.y += 1;
                }
                b'#' => {
                    map.insert(&p, Tile::Wall);
                    p.x += 1;
                }
                b'O' => {
                    map.insert(&p, Tile::Box);
                    p.x += 1;
                }
                b'@' => {
                    start = Some(p);
                    p.x += 1;
                }
                _ => {
                    p.x += 1;
                }
            }
        }

        let start = start.ok_or(anyhow!("failed to find robot"))?;

        let moves = moves_chunk
            .bytes()
            .filter(|&b| b != b'\n')
            .map(Direction::try_from)
            .collect::<Anyhow<Vec<_>>>()?;

        Ok(Self { map, start, moves })
    }

    fn part1(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self.clone().solve())
    }

    fn part2(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self.expanded().solve())
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
