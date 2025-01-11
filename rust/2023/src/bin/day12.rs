type Cache = FxHashMap<Record, usize>;

fn permutations(conditions: &[Condition], groups: &[u8], cache: &mut Cache) -> usize {
    if conditions.is_empty() && groups.is_empty() {
        return 1;
    }

    if let Some(&result) = cache.get(&Record::new(conditions, groups)) {
        return result;
    }

    let mut perms = 0;

    if let Some(&group) = groups.first() {
        if conditions.len() >= group as usize {
            let can_match_group = conditions[..group as usize]
                .iter()
                .all(|&c| c == Condition::Damaged || c == Condition::Unknown);

            match (can_match_group, conditions.get(group as usize)) {
                (true, Some(&Condition::Unknown)) => {
                    perms += permutations(
                        &[&[Condition::Operational], &conditions[group as usize + 1..]].concat(),
                        &groups[1..],
                        cache,
                    );
                }
                (true, Some(&Condition::Operational) | None) => {
                    perms += permutations(&conditions[group as usize..], &groups[1..], cache);
                }
                _ => {}
            }
        }
    }

    if !conditions.is_empty() && conditions.first() != Some(&Condition::Damaged) {
        perms += permutations(&conditions[1..], groups, cache);
    }

    cache.insert(Record::new(conditions, groups), perms);
    perms
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
enum Condition {
    Operational,
    Damaged,
    Unknown,
}

impl TryFrom<char> for Condition {
    type Error = Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '.' => Ok(Condition::Operational),
            '#' => Ok(Condition::Damaged),
            '?' => Ok(Condition::Unknown),
            _ => Err(anyhow!("failed to parse char: '{c}'")),
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
struct Record {
    conditions: Vec<Condition>,
    groups: Vec<u8>,
}

impl TryFrom<&str> for Record {
    type Error = Error;

    fn try_from(line: &str) -> Result<Self, Self::Error> {
        let (conditions, groups) = line
            .split_once(' ')
            .ok_or(anyhow!("failed to split line"))?;

        let conditions = conditions
            .chars()
            .filter_map(|c| Condition::try_from(c).ok())
            .collect();

        let groups = groups
            .split(',')
            .map(|n| n.parse())
            .collect::<ParseIntResult<_>>()?;

        Ok(Self { conditions, groups })
    }
}

impl std::hash::Hash for Record {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.conditions.hash(state);
        self.groups.hash(state);
    }
}

impl Record {
    fn new(conditions: &[Condition], groups: &[u8]) -> Self {
        Self {
            conditions: conditions.to_vec(),
            groups: groups.to_vec(),
        }
    }

    fn solve(&self) -> usize {
        permutations(&self.conditions, &self.groups, &mut Cache::default())
    }

    fn unfolded(&self, folds: usize) -> Self {
        let conditions = &[&self.conditions[..], &[Condition::Unknown]]
            .concat()
            .repeat(folds);

        Self::new(
            &conditions[..conditions.len() - 1],
            &self.groups.repeat(folds),
        )
    }
}

#[derive(Clone)]
struct Solution {
    records: Vec<Record>,
}

impl Solver for Solution {
    fn new(input: &str) -> Anyhow<Self> {
        Ok(Self {
            records: input
                .par_lines()
                .map(Record::try_from)
                .collect::<Anyhow<Vec<_>>>()?,
        })
    }

    fn part1(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self.records.par_iter().map(Record::solve).sum::<usize>())
    }

    fn part2(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self
            .records
            .par_iter()
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
