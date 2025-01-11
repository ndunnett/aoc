use std::{cmp::Ordering, collections::BinaryHeap};

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
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
    const ALL: [Self; 4] = [Self::N, Self::E, Self::S, Self::W];

    fn next_move(&self, p: Point, d: Self, size: usize) -> Option<(Point, Self)> {
        match (d, self) {
            (Self::N, Self::S) | (Self::S, Self::N) => None,
            (Self::E, Self::W) | (Self::W, Self::E) => None,
            (_, Self::N) if p.y > 0 => Some((Point::new(p.x, p.y - 1), *self)),
            (_, Self::E) if p.x < size - 1 => Some((Point::new(p.x + 1, p.y), *self)),
            (_, Self::S) if p.y < size - 1 => Some((Point::new(p.x, p.y + 1), *self)),
            (_, Self::W) if p.x > 0 => Some((Point::new(p.x - 1, p.y), *self)),
            _ => None,
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
struct State {
    p: Point,
    d: Direction,
    path: Vec<Point>,
    score: usize,
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

#[derive(Clone)]
struct Solution {
    best_score: usize,
    best_tiles: usize,
}

impl Solver for Solution {
    fn new(input: &str) -> Anyhow<Self> {
        let maze = input.bytes().collect::<Vec<_>>();
        let size = input.lines().count();
        let start = Point::new(1, size - 2);
        let end = Point::new(size - 2, 1);

        let mut queue = BinaryHeap::from_iter([State {
            p: start,
            d: Direction::E,
            path: vec![start],
            score: 0,
        }]);

        let mut seen = FxHashMap::default();
        let mut best_score = None;
        let mut best_paths = Vec::new();

        while let Some(State { p, d, path, score }) = queue.pop() {
            if p == end {
                if best_score.is_none_or(|s| s > score) {
                    best_score = Some(score);
                    best_paths.clear();
                    best_paths.push(path.clone());
                } else if best_score == Some(score) {
                    best_paths.push(path.clone());
                } else {
                    break;
                }
            }

            seen.insert((p, d), score);

            for (next_p, next_d) in Direction::ALL
                .iter()
                .filter_map(|next_d| next_d.next_move(p, d, size))
                .filter(|(p, _)| maze[p.x + p.y * (size + 1)] != b'#')
            {
                let next_score = if next_d == d { score + 1 } else { score + 1001 };

                if seen.get(&(next_p, next_d)).is_none_or(|&s| s > next_score) {
                    queue.push(State {
                        p: next_p,
                        d: next_d,
                        path: [&path[..], &[next_p]].concat(),
                        score: next_score,
                    });
                }
            }
        }

        let mut best_tiles: FxHashSet<Point> = FxHashSet::default();

        for path in best_paths {
            best_tiles.extend(path);
        }

        Ok(Self {
            best_score: best_score.ok_or(anyhow!("failed to find path"))?,
            best_tiles: best_tiles.len(),
        })
    }

    fn part1(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self.best_score)
    }

    fn part2(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self.best_tiles)
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
