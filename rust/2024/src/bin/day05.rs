use std::cmp::Ordering;

fn parse_two_digits(a: u8, b: u8) -> u8 {
    let n = a - b'0';
    (n << 1) + (n << 3) + b - b'0'
}

struct Solution {
    before: [FxHashSet<u8>; 100],
    after: [FxHashSet<u8>; 100],
    updates: Vec<Vec<u8>>,
}

impl Solution {
    fn is_sorted(&self, update: &[u8]) -> bool {
        for (a, b) in update.iter().tuple_windows() {
            if self.compare(a, b).is_eq() {
                return false;
            }
        }

        true
    }

    fn compare(&self, a: &u8, b: &u8) -> Ordering {
        if self.before[*b as usize].contains(a) || self.after[*a as usize].contains(b) {
            Ordering::Equal
        } else {
            Ordering::Less
        }
    }

    fn make_set(_: usize) -> FxHashSet<u8> {
        FxHashSet::with_capacity_and_hasher(24, FxBuildHasher)
    }
}

impl Solver for Solution {
    fn new(input: &str) -> Anyhow<Self> {
        let (rules, updates_chunk) = input
            .split_once("\n\n")
            .ok_or(anyhow!("failed to split sections"))?;

        let mut before: [_; 100] = std::array::from_fn(Self::make_set);
        let mut after: [_; 100] = std::array::from_fn(Self::make_set);

        for (a1, a2, _, b1, b2, _) in rules.bytes().chain([b'\n']).tuples() {
            let a = parse_two_digits(a1, a2);
            let b = parse_two_digits(b1, b2);
            before[a as usize].insert(b);
            after[b as usize].insert(a);
        }

        let mut updates = Vec::new();
        let mut update = Vec::new();
        let mut bytes = updates_chunk.bytes();

        while let Some(b) = bytes.next() {
            let mut num = b - b'0';

            if let Some(mut b) = bytes.next() {
                if b.is_ascii_digit() {
                    num = (num << 1) + (num << 3) + b - b'0';
                    b = bytes.next().unwrap();
                }

                update.push(num);

                if b == b'\n' {
                    updates.push(update.clone());
                    update.clear();
                }
            }
        }

        Ok(Self {
            before,
            after,
            updates,
        })
    }

    fn part1(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self
            .updates
            .iter()
            .filter_map(|update| {
                if self.is_sorted(update) {
                    Some(update[update.len() / 2] as u16)
                } else {
                    None
                }
            })
            .sum::<u16>())
    }

    fn part2(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self
            .updates
            .par_iter()
            .filter_map(|update| {
                if !self.is_sorted(update) {
                    update
                        .iter()
                        .sorted_by(|a, b| self.compare(a, b))
                        .nth(update.len() / 2)
                        .map(|n| *n as u16)
                } else {
                    None
                }
            })
            .sum::<u16>())
    }
}

aoc::solution!();

#[cfg(test)]
mod test {
    use super::{Solution, Solver};

    const INPUT: &str = r"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

    #[test]
    fn test_part1() {
        let mut solution = Solution::new(INPUT).unwrap();
        let answer = solution.part1().unwrap().to_string();
        assert_eq!(answer, "143");
    }

    #[test]
    fn test_part2() {
        let mut solution = Solution::new(INPUT).unwrap();
        let answer = solution.part2().unwrap().to_string();
        assert_eq!(answer, "123");
    }
}
