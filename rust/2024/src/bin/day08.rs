struct Solution {
    frequencies: FxHashMap<char, FxHashSet<(i64, i64)>>,
    size: i64,
}

impl Solution {
    fn in_range(&self, point: (i64, i64)) -> bool {
        0 <= point.0 && point.0 < self.size && 0 <= point.1 && point.1 < self.size
    }
}

impl Solver for Solution {
    fn new(input: &str) -> Anyhow<Self> {
        let mut frequencies: FxHashMap<char, FxHashSet<(i64, i64)>> = FxHashMap::default();

        for (y, line) in input.lines().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if ch != '.' {
                    frequencies
                        .entry(ch)
                        .or_default()
                        .insert((x as i64, y as i64));
                }
            }
        }

        Ok(Self {
            frequencies,
            size: input.lines().count() as i64,
        })
    }

    fn part1(&mut self) -> Anyhow<impl fmt::Display> {
        let mut antinodes: FxHashSet<(i64, i64)> = FxHashSet::default();

        for (_, indices) in self.frequencies.iter() {
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
        let mut antinodes: FxHashSet<(i64, i64)> = FxHashSet::default();

        for (_, indices) in self.frequencies.iter() {
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
............";

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
