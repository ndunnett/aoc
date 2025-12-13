type Coordinate = i64;
type Point = GenericPoint<2, Coordinate>;
type TilesVec = MicroVec<Point, 500, usize>;

#[derive(Clone, Copy)]
enum Edge {
    Vertical {
        x: Coordinate,
        up: Coordinate,
        down: Coordinate,
    },
    Horizontal {
        y: Coordinate,
        left: Coordinate,
        right: Coordinate,
    },
}

type EdgesVec = MicroVec<Edge, 500, usize>;

#[derive(Clone)]
struct Solution {
    tiles: Box<TilesVec>,
}

impl Solution {
    #[inline(always)]
    fn for_each_tile_pair<F: FnMut(&Point, &Point)>(&self, mut func: F) {
        self.tiles
            .iter()
            .enumerate()
            .for_each(|(i, a)| self.tiles.iter().skip(i + 1).for_each(|b| func(a, b)))
    }
}

impl Solver for Solution {
    fn new(input: &str) -> Anyhow<Self> {
        Ok(Self {
            tiles: Box::new(
                NumberParser::from(input)
                    .tuples::<(_, _)>()
                    .map(Point::from)
                    .collect(),
            ),
        })
    }

    fn part1(&mut self) -> Anyhow<impl fmt::Display> {
        let mut best = 0;

        // Iterate all red tile pairs, brute force to find the best area
        self.for_each_tile_pair(|a, b| {
            let (x_left, y_up) = (*a).into();
            let (x_right, y_down) = (*b).into();
            best = best.max(((x_right - x_left).abs() + 1) * ((y_down - y_up).abs() + 1));
        });

        Ok(best)
    }

    fn part2(&mut self) -> Anyhow<impl fmt::Display> {
        // Build edges from red tile loop
        let mut edges = Box::new(EdgesVec::new());

        for (a, b) in self.tiles.iter().circular_tuple_windows() {
            let (mut left, mut up) = (*a).into();
            let (mut right, mut down) = (*b).into();

            if left == right {
                if up > down {
                    std::mem::swap(&mut up, &mut down);
                }

                edges.push(Edge::Vertical { x: left, up, down });
            } else {
                if left > right {
                    std::mem::swap(&mut left, &mut right);
                }

                edges.push(Edge::Horizontal { y: up, left, right });
            }
        }

        let mut best = 0;

        // Iterate all red tile pairs
        self.for_each_tile_pair(|a, b| {
            // Get bounds for each axis and calculate area
            let mut bb_left = a.x();
            let mut bb_right = b.x();

            if bb_left > bb_right {
                std::mem::swap(&mut bb_left, &mut bb_right);
            }

            let mut bb_up = a.y();
            let mut bb_down = b.y();

            if bb_up > bb_down {
                std::mem::swap(&mut bb_up, &mut bb_down);
            }

            let area = (bb_right - bb_left + 1) * (bb_down - bb_up + 1);

            if area > best
                // Discard pair if any edges intersect the interior
                && !edges.iter().any(|edge| match *edge {
                    Edge::Horizontal { y, left, right } => {
                        y > bb_up && y < bb_down && left < bb_right && right > bb_left
                    }
                    Edge::Vertical { x, up, down } => {
                        x > bb_left && x < bb_right && up < bb_down && down > bb_up
                    }
                })
            {
                best = area;
            }
        });

        Ok(best)
    }
}

aoc::solution!();

#[cfg(test)]
mod test {
    use super::{Solution, Solver};

    const INPUT: &str = r"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";

    #[test]
    fn test_part1() {
        let mut solution = Solution::new(INPUT).unwrap();
        let answer = solution.part1().unwrap().to_string();
        assert_eq!(answer, "50");
    }

    #[test]
    fn test_part2() {
        let mut solution = Solution::new(INPUT).unwrap();
        let answer = solution.part2().unwrap().to_string();
        assert_eq!(answer, "24");
    }
}
