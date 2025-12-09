#![feature(portable_simd)]
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
use std::simd::prelude::*;

const LANE_SIZE: usize = 64;
const ASCII_SPLITTERS: Simd<u8, LANE_SIZE> = Simd::splat(b'^');

struct BitSet {
    data: [u64; 3],
}

impl BitSet {
    #[inline(always)]
    const fn new() -> Self {
        Self { data: [0; 3] }
    }

    #[inline(always)]
    fn contains(&self, x: usize) -> bool {
        (self.data[x >> 6] >> (x & 63)) & 1 == 1
    }

    #[inline(always)]
    fn insert_chunk(&mut self, index: usize, chunk: u64) {
        self.data[index] = chunk;
    }
}

fn solve<const N: usize>(input: &str) -> (u64, u64)
where
    [(); N.div_ceil(2)]:,
{
    let bytes = input.as_bytes();
    let mut rows = [const { BitSet::new() }; { N.div_ceil(2) }];

    // Every second row has no splitters and can be omitted
    for y in (0..N).step_by(2) {
        // Parse the row of splitters using SIMD, assumes that there are enough bytes after the last
        // parsed line (N - 1) without overrunning `input`, this allows us to vectorise the entire parse
        // and avoid a scalar loop
        let mut i = 0;

        while i * LANE_SIZE <= N {
            let start = y * (N + 1) + i * LANE_SIZE;
            let lane = Simd::from_slice(&bytes[start..start + LANE_SIZE]);
            let mask = lane.simd_eq(ASCII_SPLITTERS).to_bitmask();
            rows[y / 2].insert_chunk(i, mask);
            i += 1;
        }
    }

    let mut splitters_hit = 0;
    let mut counts = [0; N];
    counts[N / 2] = 1; // The starting point is always in the middle

    for (y, splitters) in rows.into_iter().enumerate() {
        // Only iterate over a triangular region, starting from the middle at the top
        let x_len = y * 2 + 1;
        let x_start = (N - x_len) / 2;
        let x_end = x_start + x_len;

        // Iterate from left to right, modifying elements to the left directly and carrying modifications to the
        // right over to the next iteration on `next_count`
        let mut current_count = 0;
        let mut next_count = 0;

        for x in x_start..x_end {
            if counts[x] > 0 && splitters.contains(x) {
                splitters_hit += 1;
                counts[x - 1] += counts[x];
                next_count = counts[x];
                counts[x] = 0;
            }

            counts[x] += current_count;
            current_count = next_count;
            next_count = 0;
        }
    }

    (splitters_hit, counts.into_iter().sum())
}

#[derive(Clone)]
struct Solution {
    part1: u64,
    part2: u64,
}

impl Solver for Solution {
    #[inline(always)]
    fn new(input: &str) -> Anyhow<Self> {
        let (part1, part2) = solve::<141>(input);
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
    use super::solve;

    const INPUT: &str = r".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
(extra bytes so that SIMD parsing doesn't overrun the input while parsing)
";

    #[test]
    fn test_part1() {
        assert_eq!(solve::<15>(INPUT).0, 21);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve::<15>(INPUT).1, 40);
    }
}
