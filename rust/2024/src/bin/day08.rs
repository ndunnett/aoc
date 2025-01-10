struct Solution {
    frequencies: [FxHashSet<(i64, i64)>; (b'z' - b'0') as usize],
    size: i64,
}

impl Solution {
    fn in_range(&self, point: (i64, i64)) -> bool {
        0 <= point.0 && point.0 < self.size && 0 <= point.1 && point.1 < self.size
    }
}

impl Solver for Solution {
    fn new(input: &str) -> Anyhow<Self> {
        let mut frequencies =
            std::array::from_fn(|_| FxHashSet::with_capacity_and_hasher(4, FxBuildHasher));

        let mut x = 0;
        let mut y = 0;

        for b in input.bytes() {
            if b == b'\n' {
                x = 0;
                y += 1;
            } else if b != b'.' {
                frequencies[(b - b'0') as usize].insert((x, y));
                x += 1;
            } else {
                x += 1;
            }
        }

        Ok(Self {
            frequencies,
            size: y,
        })
    }

    fn part1(&mut self) -> Anyhow<impl fmt::Display> {
        let mut antinodes = FxHashSet::with_capacity_and_hasher(2_usize.pow(9), FxBuildHasher);

        for indices in &self.frequencies {
            for ((ax, ay), (bx, by)) in indices.iter().tuple_combinations() {
                let antinode1 = (2 * ax - bx, 2 * ay - by);
                let antinode2 = (2 * bx - ax, 2 * by - ay);

                if self.in_range(antinode1) {
                    antinodes.insert(antinode1);
                }

                if self.in_range(antinode2) {
                    antinodes.insert(antinode2);
                }
            }
        }

        Ok(antinodes.len())
    }

    fn part2(&mut self) -> Anyhow<impl fmt::Display> {
        let mut antinodes = FxHashSet::with_capacity_and_hasher(2_usize.pow(11), FxBuildHasher);

        for indices in &self.frequencies {
            for ((ax, ay), (bx, by)) in indices.iter().tuple_combinations() {
                let mut antinode = (*ax, *ay);
                let dx = bx - ax;
                let dy = by - ay;

                while self.in_range(antinode) {
                    antinodes.insert(antinode);
                    antinode = (antinode.0 - dx, antinode.1 - dy);
                }

                antinode = (ax + dx, ay + dy);

                while self.in_range(antinode) {
                    antinodes.insert(antinode);
                    antinode = (antinode.0 + dx, antinode.1 + dy);
                }
            }
        }

        Ok(antinodes.len())
    }
}

aoc::solution!();

#[cfg(test)]
mod test {
    use super::{Solution, Solver};

    const INPUT: &str = r"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

    #[test]
    fn test_part1() {
        let mut solution = Solution::new(INPUT).unwrap();
        let answer = solution.part1().unwrap().to_string();
        assert_eq!(answer, "14");
    }

    #[test]
    fn test_part2() {
        let mut solution = Solution::new(INPUT).unwrap();
        let answer = solution.part2().unwrap().to_string();
        assert_eq!(answer, "34");
    }
}
