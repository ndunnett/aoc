struct Frequency {
    data: [(i8, i8); 4],
    len: u8,
}

impl Frequency {
    fn new() -> Self {
        Self {
            data: [(0, 0); 4],
            len: 0,
        }
    }

    fn push(&mut self, point: (i8, i8)) {
        self.data[self.len as usize] = point;
        self.len += 1;
    }

    fn iter(&self) -> std::slice::Iter<'_, (i8, i8)> {
        self.data[..self.len as usize].iter()
    }
}

struct Set {
    data: [u64; 50],
}

impl Set {
    fn new() -> Self {
        Self { data: [0; 50] }
    }

    fn insert(&mut self, point: (i8, i8)) {
        self.data[point.0 as usize] |= 1 << point.1 as usize;
    }

    fn len(&self) -> u32 {
        self.data.iter().map(|s| s.count_ones()).sum()
    }
}

struct Solution {
    frequencies: [Frequency; (b'z' - b'0') as usize],
    size: i8,
}

impl Solution {
    #[inline(always)]
    fn in_range(&self, point: (i8, i8)) -> bool {
        0 <= point.0 && point.0 < self.size && 0 <= point.1 && point.1 < self.size
    }
}

impl Solver for Solution {
    fn new(input: &str) -> Anyhow<Self> {
        let mut frequencies = std::array::from_fn(|_| Frequency::new());
        let mut x = 0;
        let mut y = 0;

        for b in input.bytes() {
            if b == b'\n' {
                x = 0;
                y += 1;
            } else if b != b'.' {
                frequencies[(b - b'0') as usize].push((x, y));
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
        let mut antinodes = Set::new();

        for frequency in &self.frequencies {
            for ((ax, ay), (bx, by)) in frequency.iter().tuple_combinations() {
                let antinode_a = (2 * ax - bx, 2 * ay - by);
                let antinode_b = (2 * bx - ax, 2 * by - ay);

                if self.in_range(antinode_a) {
                    antinodes.insert(antinode_a);
                }

                if self.in_range(antinode_b) {
                    antinodes.insert(antinode_b);
                }
            }
        }

        Ok(antinodes.len())
    }

    fn part2(&mut self) -> Anyhow<impl fmt::Display> {
        let mut antinodes = Set::new();

        for frequency in &self.frequencies {
            for ((ax, ay), (bx, by)) in frequency.iter().tuple_combinations() {
                let mut antinode = (*ax, *ay);
                let dx = bx - ax;
                let dy = by - ay;

                while self.in_range(antinode) {
                    antinodes.insert(antinode);
                    antinode.0 -= dx;
                    antinode.1 -= dy;
                }

                antinode = (ax + dx, ay + dy);

                while self.in_range(antinode) {
                    antinodes.insert(antinode);
                    antinode.0 += dx;
                    antinode.1 += dy;
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
