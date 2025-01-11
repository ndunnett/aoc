#[derive(Clone)]
struct Set {
    data: Vec<[u64; 3]>,
}

impl Set {
    fn new(size: usize) -> Self {
        Self {
            data: vec![[0; 3]; size],
        }
    }

    fn insert(&mut self, x: u8, y: u8) -> bool {
        let i = x % 64;
        let j = (x - i) as usize / 64;
        let n = 1 << i;

        if self.data[y as usize][j] & n == 0 {
            self.data[y as usize][j] |= n;
            true
        } else {
            false
        }
    }
}

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
    const SIDES: [Self; 4] = [Self::N, Self::E, Self::S, Self::W];

    // iterate over each corner and its adjacent edges in clockwise order
    const CORNERS: [[Self; 3]; 4] = [
        [Self::W, Self::NW, Self::N],
        [Self::N, Self::NE, Self::E],
        [Self::E, Self::SE, Self::S],
        [Self::S, Self::SW, Self::W],
    ];

    fn get_next(&self, size: usize, x: u8, y: u8) -> Option<(u8, u8)> {
        let max = size as u8 - 1;

        match self {
            Self::N if y > 0 => Some((x, y - 1)),
            Self::NE if y > 0 && x < max => Some((x + 1, y - 1)),
            Self::E if x < max => Some((x + 1, y)),
            Self::SE if y < max && x < max => Some((x + 1, y + 1)),
            Self::S if y < max => Some((x, y + 1)),
            Self::SW if y < max && x > 0 => Some((x - 1, y + 1)),
            Self::W if x > 0 => Some((x - 1, y)),
            Self::NW if y > 0 && x > 0 => Some((x - 1, y - 1)),
            _ => None,
        }
    }
}

#[derive(Clone)]
struct Region {
    area: usize,
    perimeter: usize,
    corners: usize,
}

impl Region {
    fn flood_fill(map: &[&[u8]], seen: &mut Set, x: u8, y: u8) -> Self {
        let plant = map[y as usize][x as usize];
        let mut area = 0;
        let mut perimeter = 0;
        let mut corners = 0;
        let mut queue = vec![(x, y)];

        while let Some((x, y)) = queue.pop() {
            // all plants in the region form part of the area
            area += 1;

            // look at adjacent plants, if it is the same as the current plant, add it to the flood fill queue
            // if an adjacent plant differs from the current plant, it must form part of the perimeter
            perimeter += Direction::SIDES
                .iter()
                .filter(|direction| match direction.get_next(map.len(), x, y) {
                    Some((x, y)) if map[y as usize][x as usize] == plant => {
                        if seen.insert(x, y) {
                            queue.push((x, y));
                        }

                        false
                    }
                    _ => true,
                })
                .count();

            // the region is a polygon, and the number of corners in a polygon is equal to the number of sides in a polygon
            // perform pattern recognition on each corner and its adjacent edges of the current plant to count corners of the region
            // - if the corner differs from its adjacent edges, it must be a corner of the region
            // - if the corner and its adjacent edges all differ from the current plant, it also must be a corner of the region
            corners += Direction::CORNERS
                .iter()
                .map(|corner| {
                    corner
                        .iter()
                        .map(|direction| {
                            direction
                                .get_next(map.len(), x, y)
                                .is_some_and(|(x, y)| map[y as usize][x as usize] == plant)
                        })
                        .tuples()
                        .filter(|pattern| {
                            matches!(
                                pattern,
                                (true, false, true) | (false, true, false) | (false, false, false)
                            )
                        })
                        .count()
                })
                .sum::<usize>();
        }

        Self {
            area,
            perimeter,
            corners,
        }
    }
}

#[derive(Clone)]
struct Solution {
    regions: Vec<Region>,
}

impl Solver for Solution {
    fn new(input: &str) -> Anyhow<Self> {
        let map = input
            .lines()
            .map(|line| line.as_bytes())
            .collect::<Vec<_>>();

        let mut regions = Vec::with_capacity(600);
        let mut seen = Set::new(map.len());

        // start flood fill on each plant to find each contiguous region of plants in the map
        for y in 0..(map.len() as u8) {
            for x in 0..(map[0].len() as u8) {
                if seen.insert(x, y) {
                    regions.push(Region::flood_fill(&map, &mut seen, x, y));
                }
            }
        }

        Ok(Self { regions })
    }

    fn part1(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self
            .regions
            .iter()
            .map(|region| region.area * region.perimeter)
            .sum::<usize>())
    }

    fn part2(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self
            .regions
            .iter()
            .map(|region| region.area * region.corners)
            .sum::<usize>())
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
