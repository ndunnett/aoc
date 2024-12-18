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

enum Direction {
    N,
    E,
    S,
    W,
}

impl Direction {
    const ALL: [Self; 4] = [Self::N, Self::E, Self::S, Self::W];

    fn next_move(&self, p: Point, size: u8) -> Option<Point> {
        match self {
            Self::N if p.y > 0 => Some(Point::new(p.x, p.y - 1)),
            Self::E if p.x < size => Some(Point::new(p.x + 1, p.y)),
            Self::S if p.y < size => Some(Point::new(p.x, p.y + 1)),
            Self::W if p.x > 0 => Some(Point::new(p.x - 1, p.y)),
            _ => None,
        }
    }
}

struct Solution {
    positions: Vec<Point>,
    bytes: usize,
    finish: Point,
}

impl Solution {
    const SPLIT_PATTERN: [char; 2] = [',', '\n'];

    fn find_path(&self, bytes: usize) -> Option<usize> {
        let mut map = FxHashSet::from_iter(self.positions[..bytes].iter().cloned());
        let mut queue = vec![(Point::new(0, 0), 0)];

        while let Some((point, steps)) = queue.pop() {
            if point == self.finish {
                return Some(steps);
            }

            if !map.insert(point) {
                continue;
            }

            Direction::ALL
                .iter()
                .filter_map(|d| {
                    if let Some(p) = d.next_move(point, self.finish.x) {
                        if !map.contains(&p) {
                            return Some((p, steps + 1));
                        }
                    }

                    None
                })
                .for_each(|next| queue.insert(0, next));
        }

        None
    }
}

impl Solver for Solution {
    fn new(input: &str) -> Anyhow<Self> {
        Ok(Self {
            bytes: 1024,
            finish: Point::new(70, 70),
            positions: input
                .split(Self::SPLIT_PATTERN)
                .tuples()
                .map(|(x, y)| Ok(Point::new(x.parse::<u8>()?, y.parse::<u8>()?)))
                .collect::<ParseIntResult<Vec<_>>>()?,
        })
    }

    fn part1(&mut self) -> Anyhow<impl fmt::Display> {
        self.find_path(self.bytes)
            .ok_or(anyhow!("failed to find path"))
    }

    fn part2(&mut self) -> Anyhow<impl fmt::Display> {
        let mut front = self.bytes;
        let mut back = self.positions.len() - 1;

        while front + 1 < back {
            let mid = (front + back) / 2;

            if self.find_path(mid).is_some() {
                front = mid;
            } else {
                back = mid;
            }
        }

        let byte = self.positions[front];
        Ok(format!("{},{}", byte.x, byte.y))
    }
}

aoc::solution!();

#[cfg(test)]
mod test {
    use super::{Point, Solution, Solver};

    const INPUT: &str = r"5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

    #[test]
    fn test_part1() {
        let mut solution = Solution::new(INPUT).unwrap();
        solution.finish = Point::new(6, 6);
        solution.bytes = 12;
        let answer = solution.part1().unwrap().to_string();
        assert_eq!(answer, "22");
    }

    #[test]
    fn test_part2() {
        let mut solution = Solution::new(INPUT).unwrap();
        solution.finish = Point::new(6, 6);
        solution.bytes = 12;
        let answer = solution.part2().unwrap().to_string();
        assert_eq!(answer, "6,1");
    }
}
