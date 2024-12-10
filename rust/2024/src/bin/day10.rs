struct Solution {
    trailheads: Vec<Vec<usize>>,
}

#[inline(always)]
fn check_neighbour(
    map: &[u8],
    queue: &mut Vec<(usize, u8)>,
    destinations: &mut Vec<usize>,
    last: usize,
    b: u8,
    n: usize,
) {
    if n != last && map[n] > b && map[n] - b == 1 {
        if map[n] == b'9' {
            destinations.push(n);
        } else {
            queue.push((n, map[n]));
        }
    }
}

impl Solver for Solution {
    fn new(input: &str) -> Anyhow<Self> {
        let map = input.bytes().filter(|&b| b != b'\n').collect::<Vec<_>>();
        let size = input.lines().count();

        let mut trailheads = Vec::new();
        let mut destinations = Vec::new();
        let mut queue = Vec::new();

        for (i, _) in map.iter().enumerate().filter(|(_, &b)| b == b'0') {
            let mut last = i;
            queue.push((i, b'0'));

            while let Some((i, b)) = queue.pop() {
                // look left
                if i % size > 0 {
                    check_neighbour(&map, &mut queue, &mut destinations, last, b, i - 1);
                }

                // look right
                if i % size + 1 < size {
                    check_neighbour(&map, &mut queue, &mut destinations, last, b, i + 1);
                }

                // look up
                if i / size > 0 {
                    check_neighbour(&map, &mut queue, &mut destinations, last, b, i - size);
                }

                // look down
                if i / size + 1 < size {
                    check_neighbour(&map, &mut queue, &mut destinations, last, b, i + size);
                }

                last = i;
            }

            trailheads.push(destinations.clone());
            destinations.clear();
            queue.clear();
        }

        Ok(Self { trailheads })
    }

    fn part1(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self
            .trailheads
            .iter()
            .fold(0, |acc, th| acc + th.iter().collect::<FxHashSet<_>>().len()))
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
10456732";

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
