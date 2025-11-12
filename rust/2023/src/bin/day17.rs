type Coordinate = u8;
type Point = GenericPoint<2, Coordinate>;

#[derive(Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    #[inline(always)]
    fn move_point(&self, point: &mut Point, len: Coordinate) -> bool {
        match self {
            Self::North if point.y() > 0 => *point.y_mut() -= 1,
            Self::South if point.y() < len => *point.y_mut() += 1,
            Self::East if point.x() < len => *point.x_mut() += 1,
            Self::West if point.x() > 0 => *point.x_mut() -= 1,
            _ => return false,
        };

        true
    }

    #[inline(always)]
    fn turns(&self) -> [Self; 2] {
        match self {
            Self::North | Self::South => [Self::East, Self::West],
            Self::East | Self::West => [Self::North, Self::South],
        }
    }
}

#[derive(Clone, Copy)]
struct State {
    heat: u16,
    position: Point,
    direction: Direction,
}

impl State {
    #[inline(always)]
    const fn key(&self) -> usize {
        let dir_idx = match self.direction {
            Direction::North => 0,
            Direction::East => 1,
            Direction::South => 2,
            Direction::West => 3,
        };

        (self.position.y() as usize * 141 + self.position.x() as usize) * 4 + dir_idx
    }
}

#[derive(Clone)]
struct HeatMap {
    data: Vec<Vec<u8>>,
    side_len: usize,
}

impl HeatMap {
    #[inline(always)]
    fn get(&self, point: &Point) -> u8 {
        self.data[point.y() as usize][point.x() as usize]
    }

    #[inline(always)]
    fn len(&self) -> usize {
        self.side_len
    }
}

impl From<Vec<Vec<u8>>> for HeatMap {
    fn from(data: Vec<Vec<u8>>) -> Self {
        let side_len = data.len();
        Self { data, side_len }
    }
}

type BucketVec = ArrayVec<State, 296, usize>;

#[derive(Clone, Copy)]
struct BucketQueue<const BUCKETS: usize> {
    buckets: [BucketVec; BUCKETS],
    current: usize,
    len: usize,
}

impl<const BUCKETS: usize> BucketQueue<BUCKETS> {
    #[inline(always)]
    const fn new() -> Self {
        Self {
            buckets: [BucketVec::new(); _],
            current: 0,
            len: 0,
        }
    }

    #[inline(always)]
    fn push(&mut self, state: State) {
        self.buckets[state.heat as usize % BUCKETS].push(state);
        self.len += 1;
    }

    #[inline(always)]
    fn pop(&mut self) -> Option<State> {
        if self.len == 0 {
            return None;
        }

        while self.buckets[self.current % BUCKETS].is_empty() {
            self.current += 1;
        }

        let state = unsafe {
            self.buckets
                .get_unchecked_mut(self.current % BUCKETS)
                .pop()
                .unwrap_unchecked()
        };

        self.len -= 1;
        Some(state)
    }
}

#[derive(Clone)]
struct Solution {
    heat_map: HeatMap,
    target: Point,
}

impl Solution {
    #[inline(always)]
    fn solve<const MIN_STEPS: u8, const MAX_STEPS: u8>(&self) -> u16 {
        let mut queue = BucketQueue::<128>::new();
        let mut visited = KeyMap::<usize, u16, { 141 * 141 * 4 }>::new();

        for dir in [Direction::North, Direction::West] {
            let state = State {
                heat: 0,
                position: Point::new(0, 0),
                direction: dir,
            };

            visited.insert(state.key(), 0);
            queue.push(state);
        }

        while let Some(state) = queue.pop() {
            if visited
                .get(state.key())
                .is_some_and(|&heat| heat < state.heat)
            {
                continue;
            }

            if state.position == self.target {
                return state.heat;
            }

            for next_dir in state.direction.turns() {
                let mut next_pos = state.position;
                let mut next_heat = 0;

                for step in 1..=MAX_STEPS {
                    if !next_dir.move_point(&mut next_pos, self.target.x()) {
                        break;
                    }

                    next_heat += self.heat_map.get(&next_pos) as u16;

                    if step < MIN_STEPS {
                        continue;
                    }

                    let next_state = State {
                        heat: state.heat + next_heat,
                        position: next_pos,
                        direction: next_dir,
                    };

                    if visited
                        .get(next_state.key())
                        .is_none_or(|&heat| heat > next_state.heat)
                    {
                        visited.insert(next_state.key(), next_state.heat);
                        queue.push(next_state);
                    }
                }
            }
        }

        panic!("failed to solve puzzle")
    }
}

impl Solver for Solution {
    fn new(input: &str) -> Anyhow<Self> {
        let heat_map = HeatMap::from(
            input
                .lines()
                .map(|line| line.bytes().map(|c| c - b'0').collect::<Vec<_>>())
                .collect::<Vec<_>>(),
        );

        let size = heat_map.len() as Coordinate - 1;
        let target = Point::new(size, size);

        Ok(Self { heat_map, target })
    }

    fn part1(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self.solve::<1, 3>())
    }

    fn part2(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self.solve::<4, 10>())
    }
}

aoc::solution!();

#[cfg(test)]
mod test {
    use super::{Solution, Solver};

    const INPUT: &str = r"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    #[test]
    fn test_part1() {
        let mut solution = Solution::new(INPUT).unwrap();
        let answer = solution.part1().unwrap().to_string();
        assert_eq!(answer, "102");
    }

    #[test]
    fn test_part2() {
        let mut solution = Solution::new(INPUT).unwrap();
        let answer = solution.part2().unwrap().to_string();
        assert_eq!(answer, "94");
    }
}
