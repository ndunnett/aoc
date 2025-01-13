use std::{cmp::Ordering, collections::BinaryHeap};

#[derive(Clone, Copy, PartialEq, Eq)]
struct Point {
    x: u8,
    y: u8,
}

impl Point {
    fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    N,
    E,
    S,
    W,
}

impl Direction {
    const REVERSE: [usize; 4] = [2, 3, 0, 1];

    const NEXT: [[Self; 3]; 4] = [
        [Self::N, Self::E, Self::W],
        [Self::N, Self::E, Self::S],
        [Self::E, Self::S, Self::W],
        [Self::N, Self::S, Self::W],
    ];

    const TURN: [[Self; 2]; 4] = [
        [Self::E, Self::W],
        [Self::N, Self::S],
        [Self::E, Self::W],
        [Self::N, Self::S],
    ];

    fn next_move(&self, p: &Point) -> Point {
        match self {
            Self::N => Point::new(p.x, p.y - 1),
            Self::E => Point::new(p.x + 1, p.y),
            Self::S => Point::new(p.x, p.y + 1),
            Self::W => Point::new(p.x - 1, p.y),
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
struct State {
    p: Point,
    d: Direction,
    score: u32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.score.cmp(&self.score)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Copy)]
struct Set {
    data: [[u64; 3]; 141],
}

impl Set {
    fn new() -> Self {
        Self {
            data: [[0; 3]; 141],
        }
    }

    fn insert(&mut self, p: &Point) -> bool {
        let i = p.x % 64;
        let j = (p.x - i) as usize / 64;
        let n = 1 << i;

        if self.data[p.y as usize][j] & n == 0 {
            self.data[p.y as usize][j] |= n;
            true
        } else {
            false
        }
    }

    fn contains(&self, p: &Point) -> bool {
        let i = p.x % 64;
        self.data[p.y as usize][(p.x - i) as usize / 64] & (1 << i) != 0
    }

    fn len(&self) -> usize {
        self.data
            .iter()
            .flat_map(|row| row.iter().map(|chunk| chunk.count_ones() as usize))
            .sum()
    }
}

#[derive(Clone, Copy)]
struct Map<T: Clone + Copy> {
    data: [[T; 141]; 141],
}

impl<T: Clone + Copy> Map<T> {
    fn new(default: T) -> Self {
        Self {
            data: [[default; 141]; 141],
        }
    }

    fn get(&self, p: &Point) -> T {
        self.data[p.y as usize][p.x as usize]
    }

    fn get_mut(&mut self, p: &Point) -> &mut T {
        &mut self.data[p.y as usize][p.x as usize]
    }
}

#[derive(Clone)]
struct Solution {
    start: Point,
    end: Point,
    scores: [Map<u32>; 4],
    best_score: u32,
}

impl Solver for Solution {
    fn new(input: &str) -> Anyhow<Self> {
        let mut maze = Set::new();
        let mut p = Point::new(0, 0);

        for b in input.bytes() {
            match b {
                b'\n' => {
                    p.x = 0;
                    p.y += 1;
                }
                b'#' => {
                    maze.insert(&p);
                    p.x += 1;
                }
                _ => {
                    p.x += 1;
                }
            }
        }

        let size = input.lines().count() as u8;
        let start = Point::new(1, size - 2);
        let end = Point::new(size - 2, 1);

        // use Dijkstra's algorithm to find best paths
        let mut queue = BinaryHeap::with_capacity(150);
        let mut seen = [Set::new(); 4];
        let mut scores = [Map::new(u32::MAX); 4];

        queue.push(State {
            p: start,
            d: Direction::E,
            score: 0,
        });

        while let Some(State { p, d, score }) = queue.pop() {
            let best_score = scores[d as usize].get_mut(&p);
            *best_score = (*best_score).min(score);

            if !seen[d as usize].insert(&p) {
                continue;
            }

            if p == end {
                return Ok(Self {
                    start,
                    end,
                    scores,
                    best_score: score,
                });
            }

            let next_p = d.next_move(&p);

            if !maze.contains(&next_p) {
                queue.push(State {
                    p: next_p,
                    d,
                    score: score + 1,
                });
            }

            for next_d in Direction::TURN[d as usize] {
                queue.push(State {
                    p,
                    d: next_d,
                    score: score + 1000,
                });
            }
        }

        Err(anyhow!("failed to find path"))
    }

    fn part1(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self.best_score)
    }

    fn part2(&mut self) -> Anyhow<impl fmt::Display> {
        // reverse walk scores from the end to the start with DFS, following descending scores to find the best tiles
        let mut best_tiles = Set::new();
        best_tiles.insert(&self.start);
        best_tiles.insert(&self.end);

        let mut queue = Vec::with_capacity(8);

        queue.push(State {
            p: self.end,
            d: Direction::W,
            score: self.best_score,
        });

        queue.push(State {
            p: self.end,
            d: Direction::S,
            score: self.best_score,
        });

        while let Some(State { p, d, score }) = queue.pop() {
            for next_d in Direction::NEXT[d as usize] {
                let next_p = next_d.next_move(&p);

                if next_p == self.start {
                    continue;
                }

                let next_score = self.scores[Direction::REVERSE[next_d as usize]].get(&next_p);

                if next_score < score {
                    best_tiles.insert(&next_p);

                    queue.push(State {
                        p: next_p,
                        d: next_d,
                        score: next_score,
                    });
                }
            }
        }

        Ok(best_tiles.len())
    }
}

aoc::solution!();

#[cfg(test)]
mod test {
    use super::{Solution, Solver};

    const INPUT1: &str = r"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

    const INPUT2: &str = r"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

    #[test]
    fn test_part1_1() {
        let mut solution = Solution::new(INPUT1).unwrap();
        let answer = solution.part1().unwrap().to_string();
        assert_eq!(answer, "7036");
    }

    #[test]
    fn test_part1_2() {
        let mut solution = Solution::new(INPUT2).unwrap();
        let answer = solution.part1().unwrap().to_string();
        assert_eq!(answer, "11048");
    }

    #[test]
    fn test_part2_1() {
        let mut solution = Solution::new(INPUT1).unwrap();
        let answer = solution.part2().unwrap().to_string();
        assert_eq!(answer, "45");
    }

    #[test]
    fn test_part2_2() {
        let mut solution = Solution::new(INPUT2).unwrap();
        let answer = solution.part2().unwrap().to_string();
        assert_eq!(answer, "64");
    }
}
