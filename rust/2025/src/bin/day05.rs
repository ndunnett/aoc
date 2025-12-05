#![feature(portable_simd)]
use std::simd::prelude::*;

const LANE_SIZE: usize = 8;
const NEWLINES: Simd<u8, LANE_SIZE> = Simd::splat(b'\n');

#[derive(Clone)]
struct Solution {
    ranges: [[Simd<u64, LANE_SIZE>; 2]; 11],
    ingredients: Vec<u64>,
}

impl Solver for Solution {
    fn new(input: &str) -> Anyhow<Self> {
        let bytes = input.as_bytes();

        // Use SIMD to find the `\n\n` separator, no need to check the tail because the separator is always near the middle
        let mut separator = 0;

        while separator + LANE_SIZE <= bytes.len() {
            let a = Simd::from_slice(&bytes[separator..separator + LANE_SIZE]).simd_eq(NEWLINES);
            let b = a.shift_elements_left::<1>(false);
            let c = (a & b).to_bitmask();

            if c > 0 {
                separator += c.trailing_zeros() as usize;
                break;
            }

            separator += LANE_SIZE - 1;
        }

        let ingredients = NumberParser::from(&bytes[separator + 1..]).collect();

        let mut ranges = [[Simd::splat(0); 2]; 11];
        let mut index = 0;
        let mut chunk = 0;

        // Merge overlapping ranges during parsing
        for (next_lower, next_upper) in NumberParser::<u64>::from(&bytes[..separator])
            .tuples::<(_, _)>()
            .map(|(a, b)| (a, b + 1))
            .sorted_unstable()
        {
            let prev_upper = ranges[chunk][1][index];

            if next_lower <= prev_upper {
                ranges[chunk][1][index] = next_upper.max(prev_upper);
            } else {
                index += 1;

                if index == LANE_SIZE {
                    index = 0;
                    chunk += 1;
                }

                ranges[chunk][0][index] = next_lower;
                ranges[chunk][1][index] = next_upper;
            }
        }

        Ok(Self {
            ranges,
            ingredients,
        })
    }

    fn part1(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self
            .ingredients
            .iter()
            .copied()
            .map(Simd::splat)
            .filter(|&lane| {
                self.ranges.iter().any(|[lower, upper]| {
                    lower.simd_le(lane).to_bitmask() & upper.simd_gt(lane).to_bitmask() != 0
                })
            })
            .count())
    }

    fn part2(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self
            .ranges
            .iter()
            .map(|[lower, upper]| (upper - lower).reduce_sum())
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
