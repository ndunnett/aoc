struct Solution {
    keys: Vec<u32>,
    locks: Vec<u32>,
}

impl Solution {
    const PATTERN_START: u32 = 0x0001_1111;
    const COLLISION_MASK: u32 = 0x0008_8888;
}

impl Solver for Solution {
    fn new(input: &str) -> Anyhow<Self> {
        let mut keys = Vec::new();
        let mut locks = Vec::new();

        for chunk in input.split("\n\n") {
            let mut it = chunk.as_bytes().windows(6).step_by(6);

            let is_key = it
                .next()
                .ok_or(anyhow!("failed to get first row"))?
                .iter()
                .take(5)
                .all(|&element| element == b'.');

            // represent each key/lock pattern as a window of 5x 4 bit unsigned ints
            // initialise the pattern such that each column starts at 0b0001 (ie. 0x0001_1111) so that when two patterns are summed
            // and the actual value per column exceeds 5 it can be detected by masking for the 4th bit of each column (ie. 0x0008_8888)
            //
            // for example, combining 3 + 4 => (0b0001 + 0b0011) + (0b0001 + 0b0100) == 0b1001, collision == 0b1001 & 0b1000 > 0 => true
            //              combining 3 + 2 => (0b0001 + 0b0011) + (0b0001 + 0b0010) == 0b0111, collision == 0b0111 & 0b1000 > 0 => false

            let pattern = it.take(5).fold(Self::PATTERN_START, |acc, slice| {
                acc + slice
                    .iter()
                    .take(5)
                    .enumerate()
                    .filter_map(|(i, element)| {
                        if *element == b'#' {
                            Some(1_u32 << (i * 4))
                        } else {
                            None
                        }
                    })
                    .sum::<u32>()
            });

            if is_key {
                keys.push(pattern);
            } else {
                locks.push(pattern);
            }
        }

        Ok(Self { keys, locks })
    }

    fn part1(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self
            .keys
            .iter()
            .map(|&key| {
                self.locks
                    .iter()
                    .filter(|&lock| (key + lock) & Self::COLLISION_MASK == 0)
                    .count()
            })
            .sum::<usize>())
    }

    fn part2(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(true)
    }
}

aoc::solution!(1);

#[cfg(test)]
mod test {
    use super::{Solution, Solver};

    const INPUT: &str = r"#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####
";

    #[test]
    fn test_part1() {
        let mut solution = Solution::new(INPUT).unwrap();
        let answer = solution.part1().unwrap().to_string();
        assert_eq!(answer, "3");
    }
}
