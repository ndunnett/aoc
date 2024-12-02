#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl From<(usize, usize)> for Point {
    fn from(tuple: (usize, usize)) -> Self {
        Self {
            x: tuple.0 as i32,
            y: tuple.1 as i32,
        }
    }
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    const N: Self = Self { x: 0, y: -1 };
    const S: Self = Self { x: 0, y: 1 };
    const E: Self = Self { x: 1, y: 0 };
    const W: Self = Self { x: -1, y: 0 };
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum Pipe {
    Cross,
    Vertical,
    Horizontal,
    BendNE,
    BendNW,
    BendSW,
    BendSE,
}

impl TryFrom<char> for Pipe {
    type Error = Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'S' => Ok(Self::Cross),
            '|' => Ok(Self::Vertical),
            '-' => Ok(Self::Horizontal),
            'L' => Ok(Self::BendNE),
            'J' => Ok(Self::BendNW),
            '7' => Ok(Self::BendSW),
            'F' => Ok(Self::BendSE),
            _ => Err(anyhow!("failed to parse as pipe: '{c}'")),
        }
    }
}

impl Pipe {
    fn offsets(&self) -> Vec<Point> {
        match self {
            Pipe::Vertical => vec![Point::N, Point::S],
            Pipe::BendNE => vec![Point::N, Point::E],
            Pipe::BendNW => vec![Point::N, Point::W],
            Pipe::BendSE => vec![Point::S, Point::E],
            Pipe::BendSW => vec![Point::S, Point::W],
            Pipe::Horizontal => vec![Point::E, Point::W],
            Pipe::Cross => vec![Point::W, Point::E, Point::S, Point::N],
        }
    }
}

struct Solution {
    maze: Vec<Vec<Option<Pipe>>>,
}

impl Solution {
    fn get(&self, point: &Point) -> Option<&Pipe> {
        if 0 <= point.y
            && point.y < self.maze.len() as i32
            && 0 <= point.x
            && point.x < self.maze[0].len() as i32
        {
            self.maze[point.y as usize][point.x as usize].as_ref()
        } else {
            None
        }
    }

    fn find_start(&self) -> Anyhow<Point> {
        self.maze
            .iter()
            .enumerate()
            .filter_map(|(y, row)| {
                row.iter()
                    .position(|&pipe| pipe == Some(Pipe::Cross))
                    .map(|x| Point::from((x, y)))
            })
            .next()
            .ok_or(anyhow!("failed to find starting tile"))
    }

    fn find_path(&self) -> Anyhow<HashSet<Point>> {
        let start = self.find_start()?;
        let mut seen = HashSet::from_iter([start]);
        let mut queue = vec![start];

        while let Some(point) = queue.pop() {
            let pipe = if let Some(pipe) = self.get(&point) {
                pipe
            } else {
                continue;
            };

            for offset in &pipe.offsets() {
                let edge_point = point + *offset;

                if let Some(edge_pipe) = self.get(&edge_point) {
                    let is_connected = edge_pipe
                        .offsets()
                        .into_iter()
                        .any(|offset| edge_point + offset == point);

                    if is_connected {
                        if *edge_pipe == Pipe::Cross {
                            return Ok(seen);
                        } else if !seen.contains(&edge_point) {
                            seen.insert(edge_point);
                            queue.insert(0, edge_point);
                            break;
                        }
                    }
                }
            }
        }

        Err(anyhow!("failed to find path in maze"))
    }

    fn reduced(self) -> Anyhow<Self> {
        let mut maze = vec![vec![None; self.maze[0].len()]; self.maze.len()];

        for point in &self.find_path()? {
            maze[point.y as usize][point.x as usize] = self.get(point).cloned();
        }

        Ok(Self { maze })
    }
}

impl Solver for Solution {
    fn new(input: &str) -> Anyhow<Self> {
        let maze = input
            .par_lines()
            .map(|line| {
                line.chars()
                    .map(|c| Pipe::try_from(c).ok())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        Self { maze }.reduced()
    }

    fn part1(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self
            .maze
            .par_iter()
            .map(|row| row.iter().filter(|point| point.is_some()).count())
            .sum::<usize>()
            / 2)
    }

    fn part2(&mut self) -> Anyhow<impl fmt::Display> {
        let extent = Point::from((self.maze[0].len(), self.maze.len()));

        Ok((0..=extent.y)
            .par_bridge()
            .map(|y| {
                (0..=extent.x)
                    .fold((0, false), |(enclosed, in_loop), x| {
                        match (in_loop, self.get(&Point::new(x, y))) {
                            (_, Some(Pipe::Vertical | Pipe::BendNW | Pipe::BendNE)) => {
                                (enclosed, !in_loop)
                            }
                            (true, None) => (enclosed + 1, in_loop),
                            _ => (enclosed, in_loop),
                        }
                    })
                    .0
            })
            .sum::<u32>())
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
