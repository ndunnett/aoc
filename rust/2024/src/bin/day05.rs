use std::cmp::Ordering;

fn parse_two_digits(a: u8, b: u8) -> u8 {
    let n = a - b'0';
    (n << 1) + (n << 3) + b - b'0'
}

struct Solution {
    before: FxHashMap<u8, FxHashSet<u8>>,
    after: FxHashMap<u8, FxHashSet<u8>>,
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
        if self.before.get(b).is_some_and(|after| after.contains(a))
            || self.after.get(a).is_some_and(|before| before.contains(b))
        {
            Ordering::Equal
        } else {
            Ordering::Less
        }
    }
}

impl Solver for Solution {
    fn new(input: &str) -> Anyhow<Self> {
        let (rules, updates) = input
            .split_once("\n\n")
            .ok_or(anyhow!("failed to split sections"))?;

        let mut before: FxHashMap<u8, FxHashSet<u8>> = FxHashMap::default();
        let mut after: FxHashMap<u8, FxHashSet<u8>> = FxHashMap::default();

        for (a1, a2, _, b1, b2, _) in rules.bytes().chain([b'\n']).tuples() {
            let a = parse_two_digits(a1, a2);
            let b = parse_two_digits(b1, b2);
            before.entry(a).or_default().insert(b);
            after.entry(b).or_default().insert(a);
        }

        let updates = updates
            .split_inclusive('\n')
            .map(|line| {
                line.bytes()
                    .tuples()
                    .map(|(a, b, _)| parse_two_digits(a, b))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        Ok(Self {
            before,
            after,
            updates,
        })
    }

    fn part1(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self.updates.iter().fold(0, |acc, update| {
            if self.is_sorted(update) {
                acc + update[update.len() / 2] as u32
            } else {
                acc
            }
        }))
    }

    fn part2(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self.updates.iter().fold(0, |acc, update| {
            if self.is_sorted(update) {
                acc
            } else {
                acc + *update
                    .iter()
                    .sorted_by(|a, b| self.compare(a, b))
                    .nth(update.len() / 2)
                    .unwrap_or(&0) as u32
            }
        }))
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
