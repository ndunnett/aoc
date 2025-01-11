#[derive(Clone)]
struct Solution {
    matches: Vec<usize>,
}

impl Solver for Solution {
    fn new(input: &str) -> Anyhow<Self> {
        let matches = input
            .lines()
            .map(|line| {
                line.splitn(3, [':', '|'])
                    .skip(1)
                    .map(|chunk| {
                        chunk
                            .split_whitespace()
                            .map(|n| n.parse::<usize>())
                            .collect::<ParseIntResult<Vec<_>>>()
                    })
                    .collect::<ParseIntResult<Vec<_>>>()
                    .map(|parsed| {
                        let [winners, picks] = [&parsed[0], &parsed[1]];

                        winners
                            .iter()
                            .filter(|winner| picks.contains(winner))
                            .count()
                    })
            })
            .collect::<ParseIntResult<Vec<_>>>()?;

        Ok(Self { matches })
    }

    fn part1(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self.matches.iter().fold(0, |acc, &matches| {
            if matches > 0 {
                acc + 2_u32.pow(matches as u32 - 1)
            } else {
                acc
            }
        }))
    }

    fn part2(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self
            .matches
            .iter()
            .enumerate()
            .fold(
                (0, HashMap::new()),
                |(acc, mut copy_counts), (i, &matches)| {
                    let n = copy_counts.get(&i).unwrap_or(&0) + 1;

                    if matches > 0 {
                        for j in 1..=matches {
                            copy_counts
                                .entry(i + j)
                                .and_modify(|count| *count += n)
                                .or_insert(n);
                        }
                    }

                    (acc + n, copy_counts)
                },
            )
            .0)
    }
}

aoc::solution!();

#[cfg(test)]
mod test {
    use super::{Solution, Solver};

    const INPUT: &str = r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_part1() {
        let mut solution = Solution::new(INPUT).unwrap();
        let answer = solution.part1().unwrap().to_string();
        assert_eq!(answer, "13");
    }

    #[test]
    fn test_part2() {
        let mut solution = Solution::new(INPUT).unwrap();
        let answer = solution.part2().unwrap().to_string();
        assert_eq!(answer, "30");
    }
}
