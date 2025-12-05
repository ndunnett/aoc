#[derive(Clone)]
struct Solution {
    fresh_ranges: Vec<(u64, u64)>,
    ingredients: Vec<u64>,
}

impl Solver for Solution {
    fn new(input: &str) -> Anyhow<Self> {
        let (a, b) = input.split_once("\n\n").unwrap();
        let ingredients = NumberParser::from(b).collect();
        let fresh_ranges_raw = NumberParser::<u64>::from(a)
            .tuples::<(_, _)>()
            .sorted_unstable();

        let mut fresh_ranges = Vec::with_capacity(90);

        for (lower, upper) in fresh_ranges_raw {
            if let Some((_, prev_upper)) = fresh_ranges.last_mut()
                && lower <= *prev_upper + 1
            {
                *prev_upper = upper.max(*prev_upper);
            } else {
                fresh_ranges.push((lower, upper));
            }
        }

        Ok(Self {
            fresh_ranges,
            ingredients,
        })
    }

    fn part1(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self
            .ingredients
            .iter()
            .filter(|&&ingredient| {
                self.fresh_ranges
                    .iter()
                    .any(|&(lower, upper)| lower <= ingredient && ingredient <= upper)
            })
            .count())
    }

    fn part2(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self
            .fresh_ranges
            .iter()
            .map(|(lower, upper)| upper - lower + 1)
            .sum::<u64>())
    }
}

aoc::solution!();

#[cfg(test)]
mod test {
    use super::{Solution, Solver};

    const INPUT: &str = r"3-5
10-14
16-20
12-18

1
5
8
11
17
32
";

    #[test]
    fn test_part1() {
        let mut solution = Solution::new(INPUT).unwrap();
        let answer = solution.part1().unwrap().to_string();
        assert_eq!(answer, "3");
    }

    #[test]
    fn test_part2() {
        let mut solution = Solution::new(INPUT).unwrap();
        let answer = solution.part2().unwrap().to_string();
        assert_eq!(answer, "14");
    }
}
