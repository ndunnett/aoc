#![feature(portable_simd)]
use std::simd::prelude::*;

/// Number of elements to a lane for SIMD
const LANE_SIZE: usize = 16;

/// LUT for precomputed splats for each digit, where SPLATS[i] = Simd::splat(i)
const SPLATS: [Simd<u8, LANE_SIZE>; 10] = {
    let mut splats = [Simd::splat(0); 10];
    let mut i = 1;

    while i < 10 {
        splats[i] = Simd::splat(i as u8);
        i += 1;
    }

    splats
};

/// Finds the first occurance of the biggest digit within a slice, returns the index and value
fn find_biggest_digit(slice: &[u8]) -> (usize, u8) {
    let mut value = 0;
    let mut best = 0;
    let mut i = 0;

    // Use SIMD to check multiple values in parallel until the tail is too small to fill a lane
    while i + LANE_SIZE <= slice.len() {
        let lane = Simd::from_slice(&slice[i..i + LANE_SIZE]);

        // Get the biggest digit within the lane
        let chunk_max = lane.reduce_max();

        if chunk_max > value {
            // Mask out the values equal to the biggest digit
            let mask = lane.simd_eq(SPLATS[chunk_max as usize]);

            // Find the index of the first value
            let offset = mask.to_bitmask().trailing_zeros() as usize;

            value = chunk_max;
            best = i + offset;

            if value == 9 {
                return (best, value);
            }
        }

        i += LANE_SIZE;
    }

    // Check the tail using a scalar loop
    while i < slice.len() {
        let current = slice[i];

        if current > value {
            value = current;
            best = i;

            if current == 9 {
                return (best, value);
            }
        }

        i += 1;
    }

    (best, value)
}

/// Inner solution struct, generic dimensions to allow testing
#[derive(Clone)]
struct Batteries<const W: usize, const L: usize> {
    banks: Box<[[u8; W]; L]>,
}

impl<const W: usize, const L: usize> Batteries<W, L> {
    fn parse(input: &str) -> Self {
        let bytes = input.as_bytes();
        let mut banks = Box::new([[0; W]; L]);

        for y in 0..L {
            for x in 0..W {
                banks[y][x] = bytes[y * (W + 1) + x] - b'0';
            }
        }

        Self { banks }
    }

    /// Solves the largest joltage for `N` batteries
    fn joltage<const N: usize>(&self) -> u64 {
        let mut total = 0;

        for bank in self.banks.iter() {
            let mut joltage = 0;
            let mut last_i = 0;

            // Iterate increasing ranges for each battery, the first battery must be at least `N` away
            // from the final element, each following battery must be after the preceding battery
            for stop in 1 + W - N..1 + W {
                let (i, value) = find_biggest_digit(&bank[last_i..stop]);
                last_i += i + 1;
                joltage = joltage * 10 + value as u64;
            }

            total += joltage;
        }

        total
    }
}

/// Wrapper around the inner solution, because this can't be generic
#[derive(Clone)]
#[repr(transparent)]
struct Solution(Batteries<100, 200>);

impl Solver for Solution {
    #[inline(always)]
    fn new(input: &str) -> Anyhow<Self> {
        Ok(Self(Batteries::<100, 200>::parse(input)))
    }

    #[inline(always)]
    fn part1(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self.0.joltage::<2>())
    }

    #[inline(always)]
    fn part2(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self.0.joltage::<12>())
    }
}

aoc::solution!();

#[cfg(test)]
mod test {
    use super::Batteries;

    const INPUT: &str = r"987654321111111
811111111111119
234234234234278
818181911112111
";

    #[test]
    fn test_part1() {
        assert_eq!(Batteries::<15, 4>::parse(INPUT).joltage::<2>(), 357);
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            Batteries::<15, 4>::parse(INPUT).joltage::<12>(),
            3121910778619
        );
    }
}
