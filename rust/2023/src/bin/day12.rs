#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
enum Condition {
    Operational,
    Damaged,
    Unknown,
}

impl Condition {
    fn from_char(c: char) -> Option<Condition> {
        match c {
            '.' => Some(Condition::Operational),
            '#' => Some(Condition::Damaged),
            '?' => Some(Condition::Unknown),
            _ => None,
        }
    }
}

type Cache = HashMap<Record, usize>;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Record {
    conditions: Vec<Condition>,
    groups: Vec<usize>,
}

impl Record {
    fn new(conditions: &[Condition], groups: &[usize]) -> Record {
        Record {
            conditions: conditions.to_vec(),
            groups: groups.to_vec(),
        }
    }

    fn parse(line: &str) -> Record {
        let split = line.split_once(' ').unwrap();
        Record {
            conditions: split.0.chars().filter_map(Condition::from_char).collect(),
            groups: split.1.split(',').map(|n| n.parse().unwrap()).collect(),
        }
    }

    fn solve_part(conditions: &[Condition], groups: &[usize], cache: &mut Cache) -> usize {
        if conditions.is_empty() && groups.is_empty() {
            return 1;
        }

        if let Some(&result) = cache.get(&Record::new(conditions, groups)) {
            return result;
        }

        let mut permutations = 0;

        if let Some(&group) = groups.first() {
            if conditions.len() >= group
                && conditions[..group]
                    .iter()
                    .all(|c| matches!(c, Condition::Damaged | Condition::Unknown))
                && *conditions.get(group).unwrap_or(&Condition::Unknown) != Condition::Damaged
            {
                if conditions.get(group) == Some(&Condition::Unknown) {
                    permutations += Record::solve_part(
                        &[&[Condition::Operational], &conditions[group + 1..]].concat(),
                        &groups[1..],
                        cache,
                    );
                } else {
                    permutations += Record::solve_part(&conditions[group..], &groups[1..], cache);
                }
            }
        }

        if conditions.first().is_some_and(|&c| c != Condition::Damaged) {
            permutations += Record::solve_part(&conditions[1..], groups, cache);
        }

        cache.insert(Record::new(conditions, groups), permutations);
        permutations
    }

    fn solve(&self) -> usize {
        Record::solve_part(&self.conditions, &self.groups, &mut Cache::new())
    }

    fn unfolded(&self, folds: usize) -> Record {
        let conditions = &[&self.conditions[..], &[Condition::Unknown]]
            .concat()
            .repeat(folds);

        Record::new(
            &conditions[..conditions.len() - 1],
            &self.groups.repeat(folds),
        )
    }
}

pub struct Solution {
    records: Vec<Record>,
}

impl Solver for Solution {
    fn new(input: &str) -> Anyhow<Self> {
        Ok(Self {
            records: input.lines().map(Record::parse).collect(),
        })
    }

    fn part1(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self.records.iter().map(Record::solve).sum::<usize>())
    }

    fn part2(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self
            .records
            .iter()
            .map(|r| r.unfolded(5).solve())
            .sum::<usize>())
    }
}

aoc::solution!();

#[cfg(test)]
mod test {
    use super::{Solution, Solver};

    const INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn test_part1() {
        let mut solution = Solution::new(INPUT).unwrap();
        let answer = solution.part1().unwrap().to_string();
        assert_eq!(answer, "21");
    }

    #[test]
    fn test_part2() {
        let mut solution = Solution::new(INPUT).unwrap();
        let answer = solution.part2().unwrap().to_string();
        assert_eq!(answer, "525152");
    }
}
