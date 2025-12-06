#![feature(portable_simd)]
use std::simd::prelude::*;

const LANE_SIZE: usize = 32;
const NEWLINES: Simd<u8, LANE_SIZE> = Simd::splat(b'\n');

#[derive(Clone, Copy)]
enum Operator {
    Add,
    Mul,
}

#[derive(Clone)]
struct Solution {
    part1: usize,
    part2: usize,
}

impl Solver for Solution {
    fn new(input: &str) -> Anyhow<Self> {
        let bytes = input.as_bytes();
        let file_end = bytes.len();

        // Use SIMD to find the first new line
        let mut first_new_line = 0;

        while first_new_line + LANE_SIZE <= bytes.len() {
            let lane = Simd::from_array(
                bytes[first_new_line..first_new_line + LANE_SIZE]
                    .try_into()
                    .unwrap(),
            );

            let mask = lane.simd_eq(NEWLINES).to_bitmask();

            if mask > 0 {
                first_new_line += mask.trailing_zeros() as usize;
                break;
            }

            first_new_line += LANE_SIZE;
        }

        let line_len = first_new_line + 1;
        let operator_line_start = file_end - line_len;
        let operand_lines = operator_line_start / line_len;

        let mut part1 = 0;
        let mut part2 = 0;
        let mut start = 0;
        let mut len = 1;

        while operator_line_start + start + len < file_end {
            let operator = match bytes[operator_line_start + start] {
                b'+' => Operator::Add,
                b'*' => Operator::Mul,
                _ => unreachable!(),
            };

            while bytes[operator_line_start + start + len] == b' ' {
                len += 1;
            }

            if bytes[operator_line_start + start + len] == b'\n' {
                len += 1;
            }

            let mut _part1 = match operator {
                Operator::Add => 0,
                Operator::Mul => 1,
            };
            let mut _part2 = _part1;

            for line in 0..operand_lines {
                let mut operand = 0;
                let i = line * line_len + start;

                for byte in &bytes[i..i + len] {
                    if byte.is_ascii_digit() {
                        operand = operand * 10 + (byte - b'0') as usize;
                    } else if operand > 0 {
                        break;
                    }
                }

                match operator {
                    Operator::Add => _part1 += operand,
                    Operator::Mul => _part1 *= operand,
                }
            }

            part1 += _part1;

            for col in (0..len - 1).rev() {
                let mut operand = 0;

                for line in 0..operand_lines {
                    let byte = bytes[line * line_len + start + col];

                    if byte.is_ascii_digit() {
                        operand = operand * 10 + (byte - b'0') as usize;
                    } else if operand > 0 {
                        break;
                    }
                }

                match operator {
                    Operator::Add => _part2 += operand,
                    Operator::Mul => _part2 *= operand,
                }
            }

            part2 += _part2;

            start += len;
            len = 1;
        }

        Ok(Self { part1, part2 })
    }

    #[inline(always)]
    fn part1(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self.part1)
    }

    #[inline(always)]
    fn part2(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self.part2)
    }
}

aoc::solution!();

#[cfg(test)]
mod test {
    use super::{Solution, Solver};

    const INPUT: &str = r"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
";

    #[test]
    fn test_part1() {
        let mut solution = Solution::new(INPUT).unwrap();
        let answer = solution.part1().unwrap().to_string();
        assert_eq!(answer, "4277556");
    }

    #[test]
    fn test_part2() {
        let mut solution = Solution::new(INPUT).unwrap();
        let answer = solution.part2().unwrap().to_string();
        assert_eq!(answer, "3263827");
    }
}
