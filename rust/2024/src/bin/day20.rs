#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    x: u8,
    y: u8,
}

impl Point {
    fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }

    fn distance(&self, other: &Point) -> u16 {
        self.x.abs_diff(other.x) as u16 + self.y.abs_diff(other.y) as u16
    }
}

enum Direction {
    N,
    E,
    S,
    W,
}

impl Direction {
    const ALL: [Self; 4] = [Self::N, Self::E, Self::S, Self::W];

    fn next_move(&self, p: &Point, size: u8) -> Option<Point> {
        match self {
            Self::N if p.y > 0 => Some(Point::new(p.x, p.y - 1)),
            Self::E if p.x + 1 < size => Some(Point::new(p.x + 1, p.y)),
            Self::S if p.y + 1 < size => Some(Point::new(p.x, p.y + 1)),
            Self::W if p.x > 0 => Some(Point::new(p.x - 1, p.y)),
            _ => None,
        }
    }
}

#[derive(Clone)]
struct Solution {
    track: Vec<(u16, Point)>,
}

impl Solution {
    fn find_cheats(&self, min_time: u16, max_distance: u16) -> usize {
        self.track
            .par_iter()
            .take(self.track.len() - min_time as usize)
            .map(|(at, ap)| {
                self.track
                    .iter()
                    .skip((*at + min_time) as usize)
                    .filter(|(bt, bp)| {
                        let d = ap.distance(bp);
                        bt > at && bt - at - d >= min_time && d <= max_distance
                    })
                    .count()
            })
            .sum()
    }
}

impl Solver for Solution {
    fn new(input: &str) -> Anyhow<Self> {
        let size = input.lines().count() as u8 - 2;
        let mut walls = FxHashSet::default();
        let mut start = None;
        let mut finish = None;

        for (y, line) in (0..size).zip(input.lines().skip(1)) {
            for (x, b) in (0..size).zip(line.bytes().skip(1)) {
                match b {
                    b'#' => {
                        walls.insert(Point::new(x, y));
                    }
                    b'S' => {
                        start = Some(Point::new(x, y));
                    }
                    b'E' => finish = Some(Point::new(x, y)),
                    _ => {}
                }
            }
        }

        let start = start.ok_or(anyhow!("failed to find start"))?;
        let finish = finish.ok_or(anyhow!("failed to find finish"))?;

        let mut track = vec![(0, start)];

        while let Some(p) = Direction::ALL
            .iter()
            .filter_map(|d| d.next_move(&track.last()?.1, size))
            .find(|p| !walls.contains(p) && (track.len() < 2 || track[track.len() - 2].1 != *p))
        {
            track.push((track.len() as u16, p));

            if p == finish {
                break;
            }
        }

        Ok(Self { track })
    }

    fn part1(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self.find_cheats(100, 2))
    }

    fn part2(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self.find_cheats(100, 20))
    }
}

aoc::solution!();

#[cfg(test)]
mod test {
    use super::{Solution, Solver};

    const INPUT: &str = r"###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

    #[test]
    fn test_part1() {
        let solution = Solution::new(INPUT).unwrap();
        let answer = solution.find_cheats(2, 2);
        assert_eq!(answer, 44);
    }

    #[test]
    fn test_part2() {
        let solution = Solution::new(INPUT).unwrap();
        let answer = solution.find_cheats(50, 20);
        assert_eq!(answer, 285);
    }
}
