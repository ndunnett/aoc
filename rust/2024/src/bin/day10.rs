#[derive(Copy, Clone)]
struct Trailhead {
    destinations: [u16; 40],
    len: u8,
}

impl Trailhead {
    fn new() -> Self {
        Self {
            destinations: [0; 40],
            len: 0,
        }
    }

    fn push(&mut self, n: u16) {
        self.destinations[self.len as usize] = n;
        self.len += 1;
    }

    fn len(&self) -> usize {
        self.len as usize
    }

    fn unique(&self) -> usize {
        self.destinations
            .iter()
            .take(self.len())
            .sorted_unstable()
            .dedup()
            .count()
    }
}

struct Solution {
    trailheads: Vec<Trailhead>,
}

impl Solver for Solution {
    fn new(input: &str) -> Anyhow<Self> {
        let line_len = input.bytes().take_while_inclusive(|&b| b != b'\n').count();
        let mut trailheads = Vec::with_capacity(400);
        let mut queue = Vec::new();

        // perform DFS from every '0' node, ie. each trailhead
        for i in input.bytes().positions(|b| b == b'0') {
            let mut last = i;
            let mut trailhead = Trailhead::new();
            queue.push((i, b'0'));

            while let Some((i, b)) = queue.pop() {
                macro_rules! check_neighbour {
                    ($n:expr) => {
                        // only continue if not backtracking and next is 1 bigger than current
                        if $n != last && $n < input.len() && input.as_bytes()[$n] == b + 1 {
                            // if current is '8', then next must be '9' and therefore a destination of the trailhead
                            if b == b'8' {
                                trailhead.push($n as u16);
                            } else {
                                queue.push(($n, input.as_bytes()[$n]));
                            }
                        }
                    };
                }

                check_neighbour!(i - 1); // look left
                check_neighbour!(i + 1); // look right
                check_neighbour!(i - line_len); // look up
                check_neighbour!(i + line_len); // look down
                last = i;
            }

            trailheads.push(trailhead);
            queue.clear();
        }

        Ok(Self { trailheads })
    }

    fn part1(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self.trailheads.iter().fold(0, |acc, th| acc + th.unique()))
    }

    fn part2(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self.trailheads.iter().fold(0, |acc, th| acc + th.len()))
    }
}

aoc::solution!();

#[cfg(test)]
mod test {
    use super::{Solution, Solver};

    const INPUT: &str = r"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";

    #[test]
    fn test_part1() {
        let mut solution = Solution::new(INPUT).unwrap();
        let answer = solution.part1().unwrap().to_string();
        assert_eq!(answer, "36");
    }

    #[test]
    fn test_part2() {
        let mut solution = Solution::new(INPUT).unwrap();
        let answer = solution.part2().unwrap().to_string();
        assert_eq!(answer, "81");
    }
}
