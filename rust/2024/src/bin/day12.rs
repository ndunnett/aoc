enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl Direction {
    fn move_(&self, x: i32, y: i32) -> (i32, i32) {
        match self {
            Self::N => (x, y - 1),
            Self::NE => (x + 1, y - 1),
            Self::E => (x + 1, y),
            Self::SE => (x + 1, y + 1),
            Self::S => (x, y + 1),
            Self::SW => (x - 1, y + 1),
            Self::W => (x - 1, y),
            Self::NW => (x - 1, y - 1),
        }
    }

    const SIDES: [Self; 4] = [Self::N, Self::E, Self::S, Self::W];

    const CORNERS: [[Self; 3]; 4] = [
        [Self::W, Self::NW, Self::N],
        [Self::N, Self::NE, Self::E],
        [Self::E, Self::SE, Self::S],
        [Self::S, Self::SW, Self::W],
    ];
}

fn get(map: &[&[u8]], x: i32, y: i32) -> Option<u8> {
    if 0 <= x && x < map[0].len() as i32 && 0 <= y && y < map.len() as i32 {
        Some(map[y as usize][x as usize])
    } else {
        None
    }
}

struct Shape {
    area: usize,
    perimeter: usize,
    corners: usize,
}

impl Shape {
    fn fill(map: &[&[u8]], seen: &mut FxHashSet<(i32, i32)>, x: i32, y: i32) -> Self {
        let plant = get(map, x, y).expect("failed to get plant");
        let mut area = 0;
        let mut perimeter = 0;
        let mut corners = 0;
        let mut queue = vec![(x, y)];

        while let Some((x, y)) = queue.pop() {
            area += 1;

            for direction in Direction::SIDES {
                let (next_x, next_y) = direction.move_(x, y);

                match get(map, next_x, next_y) {
                    Some(next_plant) if next_plant == plant => {
                        if seen.insert((next_x, next_y)) {
                            queue.push((next_x, next_y));
                        }
                    }
                    _ => perimeter += 1,
                }
            }

            for corner in Direction::CORNERS {
                let pat = corner
                    .iter()
                    .map(|direction| {
                        let (next_x, next_y) = direction.move_(x, y);

                        match get(map, next_x, next_y) {
                            Some(next_plant) if next_plant == plant => 1,
                            _ => 0,
                        }
                    })
                    .collect::<Vec<_>>();

                if matches!(pat[..], [1, 0, 1] | [0, 0, 0] | [0, 1, 0]) {
                    corners += 1;
                }
            }
        }

        Self {
            area,
            perimeter,
            corners,
        }
    }
}

struct Solution {
    shapes: Vec<Shape>,
}

impl Solver for Solution {
    fn new(input: &str) -> Anyhow<Self> {
        let map = input
            .lines()
            .map(|line| line.as_bytes())
            .collect::<Vec<_>>();

        let mut shapes = Vec::new();
        let mut seen = FxHashSet::default();

        for y in 0..(map.len() as i32) {
            for x in 0..(map[0].len() as i32) {
                if seen.insert((x, y)) {
                    shapes.push(Shape::fill(&map, &mut seen, x, y));
                }
            }
        }

        Ok(Self { shapes })
    }

    fn part1(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self
            .shapes
            .iter()
            .fold(0, |acc, shape| acc + shape.area * shape.perimeter))
    }

    fn part2(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self
            .shapes
            .iter()
            .fold(0, |acc, shape| acc + shape.area * shape.corners))
    }
}

aoc::solution!();

#[cfg(test)]
mod test {
    use super::{Solution, Solver};

    const INPUT1: &str = r"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    const INPUT2: &str = r"EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";

    const INPUT3: &str = r"AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";

    #[test]
    fn test_part1() {
        let mut solution = Solution::new(INPUT1).unwrap();
        let answer = solution.part1().unwrap().to_string();
        assert_eq!(answer, "1930");
    }

    #[test]
    fn test_part2_1() {
        let mut solution = Solution::new(INPUT2).unwrap();
        let answer = solution.part2().unwrap().to_string();
        assert_eq!(answer, "236");
    }

    #[test]
    fn test_part2_2() {
        let mut solution = Solution::new(INPUT3).unwrap();
        let answer = solution.part2().unwrap().to_string();
        assert_eq!(answer, "368");
    }

    #[test]
    fn test_part2_3() {
        let mut solution = Solution::new(INPUT1).unwrap();
        let answer = solution.part2().unwrap().to_string();
        assert_eq!(answer, "1206");
    }
}
