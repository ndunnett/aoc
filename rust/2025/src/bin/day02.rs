/// LUT for powers of 10, where POW10[n] = 10^n
const POW10: [u64; 20] = {
    let mut pow10 = [1u64; 20];
    let mut i = 1;

    while i < 20 {
        pow10[i] = pow10[i - 1] * 10;
        i += 1;
    }

    pow10
};

/// Count how many digits are in a decimal number
#[inline(always)]
const fn count_digits(n: u64) -> usize {
    (n.ilog10() + 1) as usize
}

/// Generate a mask to be used for generating numbers with repeating patterns
///
/// f(d, k) = (10^(d*k) - 1) / (10^d - 1)
/// where d = number of digits in the pattern,  k = number of repeats
///
/// e.g. to repeat the pattern 123, 5 times:
/// let d = 3,  k = 5:
/// f(5, 3) = 1001001001001
/// 123 * 1001001001001 = 123123123123123
#[inline(always)]
const fn repeater_mask(d: usize, k: usize) -> u64 {
    ((POW10[d * k]) - 1) / ((POW10[d]) - 1)
}

/// Find IDs between `first` and `last` with `digits` number of digits, that consists of a pattern
/// that repeats `repeats` number of times, and then apply function `f` to the ID.
#[inline(always)]
fn solve<F: FnMut(u64)>(first: u64, last: u64, digits: usize, repeats: usize, mut f: F) {
    // Generate a mask to render patterns on
    let pattern_size = digits / repeats;
    let mask = repeater_mask(pattern_size, repeats);

    // Determine the bounds for patterns to generate such that they all fit within the given range
    let start = first.div_ceil(mask);
    let end = last / mask;

    // Iterate over patterns from start to end, accepting results that are the correct number of digits and applying `f`
    let mut p = start;

    while p <= end {
        let id = p * mask;

        if count_digits(id) == digits {
            f(id);
        }

        p += 1;
    }
}

#[derive(Clone)]
struct Solution {
    ranges: Vec<(u64, u64)>,
}

impl Solver for Solution {
    fn new(input: &str) -> Anyhow<Self> {
        Ok(Self {
            ranges: NumberParser::<u64>::from(input).tuples().collect(),
        })
    }

    fn part1(&mut self) -> Anyhow<impl fmt::Display> {
        let mut sum = 0;

        for &(first, last) in &self.ranges {
            // We assume that the first and last ID won't be more then 10x different
            // The number of digits could be different, but only one of them will be even
            let d_first = count_digits(first);
            let d_last = count_digits(last);

            let digits = if d_first.is_multiple_of(2) {
                d_first
            } else if d_last.is_multiple_of(2) {
                d_last
            } else {
                continue;
            };

            solve(first, last, digits, 2, |id| sum += id);
        }

        Ok(sum)
    }

    fn part2(&mut self) -> Anyhow<impl fmt::Display> {
        let mut sum = 0;

        for &(first, last) in &self.ranges {
            let mut candidates = MicroVec::<u64, 200, usize>::new();

            for digits in count_digits(first)..=count_digits(last) {
                // Iterate over varying numbers of repeats that cleanly fit into the number of digits
                for repeats in 2..=digits {
                    if digits % repeats != 0 {
                        continue;
                    }

                    solve(first, last, digits, repeats, |id| candidates.push(id));
                }
            }

            // Different pattern lengths can produce duplicate IDs which need to be filtered out
            candidates.sort_unstable();
            candidates.dedup();

            for id in candidates {
                sum += id;
            }
        }

        Ok(sum)
    }
}

aoc::solution!();

#[cfg(test)]
mod test {
    use super::{Solution, Solver};

    const INPUT: &str = r"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124
";

    #[test]
    fn test_part1() {
        let mut solution = Solution::new(INPUT).unwrap();
        let answer = solution.part1().unwrap().to_string();
        assert_eq!(answer, "1227775554");
    }

    #[test]
    fn test_part2() {
        let mut solution = Solution::new(INPUT).unwrap();
        let answer = solution.part2().unwrap().to_string();
        assert_eq!(answer, "4174379265");
    }
}
