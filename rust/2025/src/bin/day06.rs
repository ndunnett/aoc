#![feature(portable_simd)]
use std::simd::prelude::*;

const LANE_SIZE: usize = 32;
const NEWLINES: Simd<u8, LANE_SIZE> = Simd::splat(b'\n');
const ASCII_ZEROES: Simd<u8, LANE_SIZE> = Simd::splat(b'0');

#[derive(Clone, Copy)]
#[repr(u8)]
enum Operator {
    Add = 0,
    Mul = 1,
}

#[derive(Clone)]
struct Solution {
    part1: u64,
    part2: u64,
}

impl Solver for Solution {
    fn new(input: &str) -> Anyhow<Self> {
        let bytes = input.as_bytes();
        let file_len = bytes.len();

        // Use SIMD to find the first new line
        let mut first_new_line = 0;

        while first_new_line + LANE_SIZE <= bytes.len() {
            let lane = Simd::from_slice(&bytes[first_new_line..first_new_line + LANE_SIZE]);
            let mask = lane.simd_eq(NEWLINES).to_bitmask();

            if mask > 0 {
                first_new_line += mask.trailing_zeros() as usize;
                break;
            }

            first_new_line += LANE_SIZE;
        }

        let line_len = first_new_line + 1;
        let operator_line_start = file_len - line_len;
        let operand_lines = operator_line_start / line_len;

        // Precompute digit table
        let mut digits = vec![0; operator_line_start];
        let mut i = 0;

        // Do the bulk with SIMD
        while i + LANE_SIZE <= operator_line_start {
            let lane = Simd::from_slice(&bytes[i..i + LANE_SIZE]);
            let values = lane - ASCII_ZEROES;

            // Safety: `i + LANE_SIZE` can never overrun the end of `digits`
            unsafe {
                std::ptr::copy_nonoverlapping(
                    values.as_array().as_ptr(),
                    digits.as_mut_ptr().add(i),
                    LANE_SIZE,
                );
            }

            i += LANE_SIZE;
        }

        // Check the tail with a scalar loop
        while i < operator_line_start {
            // Safety: `i` will never overrun `digits`
            unsafe {
                digits
                    .as_mut_ptr()
                    .add(i)
                    .write(bytes[i].wrapping_sub(b'0'));
            }

            i += 1;
        }

        let mut part1 = 0;
        let mut part2 = 0;
        let mut column_start = 0;
        let mut column_len = 1;

        // Iterate over each column of operations
        while operator_line_start + column_start + column_len < file_len {
            // Safety: the first byte at `operator line + start` is always the operator
            let operator = match bytes[operator_line_start + column_start] {
                b'+' => Operator::Add,
                b'*' => Operator::Mul,
                _ => unsafe { std::hint::unreachable_unchecked() },
            };

            // Get the width of the column by checking for spaces after the operator
            while bytes[operator_line_start + column_start + column_len] == b' ' {
                column_len += 1;
            }

            // Enforce column spacing at the end of the line
            if bytes[operator_line_start + column_start + column_len] == b'\n' {
                column_len += 1;
            }

            // Set accumulators to 0 for addition, 1 for multiplication
            let mut acc1 = operator as u64;
            let mut acc2 = acc1;

            // Parse out and process operands for part 1
            for line in 0..operand_lines {
                let mut operand = 0;
                let mut seen = false;
                let start = line * line_len + column_start;
                let end = start + column_len;

                for i in start..end {
                    // Safety: `i` will never overrun `digits`
                    let digit = unsafe { *digits.as_ptr().add(i) };

                    if digit < 10 {
                        operand = operand * 10 + digit as u64;
                        seen = true;
                    } else if seen {
                        break;
                    }
                }

                match operator {
                    Operator::Add => acc1 += operand,
                    Operator::Mul => acc1 *= operand,
                }
            }

            // Parse out and process operands for part 2
            for column in (0..column_len - 1).rev() {
                let mut operand = 0;
                let mut seen = false;

                for line in 0..operand_lines {
                    let i = line * line_len + column_start + column;

                    // Safety: `i` will never overrun `digits`
                    let digit = unsafe { *digits.as_ptr().add(i) };

                    if digit < 10 {
                        operand = operand * 10 + digit as u64;
                        seen = true;
                    } else if seen {
                        break;
                    }
                }

                match operator {
                    Operator::Add => acc2 += operand,
                    Operator::Mul => acc2 *= operand,
                }
            }

            // Add accumulators to the final answers and increment the start index
            part1 += acc1;
            part2 += acc2;
            column_start += column_len;
            column_len = 1;
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
