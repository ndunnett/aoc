fn count_permutations<'a>(
    patterns: &[&[u8]],
    design: &'a [u8],
    cache: &mut FxHashMap<&'a [u8], usize>,
) -> usize {
    if design.is_empty() {
        return 1;
    }

    if let Some(&sum) = cache.get(design) {
        return sum;
    }

    let sum = patterns
        .iter()
        .filter(|pattern| design.starts_with(pattern))
        .map(|pattern| count_permutations(patterns, &design[pattern.len()..], cache))
        .sum();

    cache.insert(design, sum);
    sum
}

struct Solution {
    counts: Vec<usize>,
}

impl Solver for Solution {
    fn new(input: &str) -> Anyhow<Self> {
        let (patterns, designs) = input
            .split_once("\n\n")
            .ok_or(anyhow!("failed to split input"))?;

        let patterns = patterns.split(", ").map(str::as_bytes).collect::<Vec<_>>();
        let mut cache = FxHashMap::default();

        Ok(Self {
            counts: designs
                .lines()
                .map(|line| count_permutations(&patterns, line.as_bytes(), &mut cache))
                .collect::<Vec<_>>(),
        })
    }

    fn part1(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self.counts.iter().filter(|&&count| count > 0).count())
    }

    fn part2(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self.counts.iter().sum::<usize>())
    }
}

aoc::solution!();

#[cfg(test)]
mod test {
    use super::{Solution, Solver};

    const INPUT: &str = r"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    #[test]
    fn test_part1() {
        let mut solution = Solution::new(INPUT).unwrap();
        let answer = solution.part1().unwrap().to_string();
        assert_eq!(answer, "6");
    }

    #[test]
    fn test_part2() {
        let mut solution = Solution::new(INPUT).unwrap();
        let answer = solution.part2().unwrap().to_string();
        assert_eq!(answer, "16");
    }
}
