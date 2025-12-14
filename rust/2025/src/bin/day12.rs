#[derive(Clone)]
struct Solution {
    part1: u16,
}

impl Solver for Solution {
    fn new(input: &str) -> Anyhow<Self> {
        let bytes = input.as_bytes();
        let mut part1 = 0;

        for (width, length, a, b, c, d, e, f) in NumberParser::<u16>::from(&bytes[96..]).tuples() {
            if width * length >= 7 * (a + b + c + d + e + f) {
                part1 += 1;
            }
        }

        Ok(Self { part1 })
    }

    fn part1(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self.part1)
    }

    fn part2(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(true)
    }
}

aoc::solution!(1);

#[cfg(test)]
mod test {
    use super::{Solution, Solver};

    const INPUT: &str = r"0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x4: 1 0 1 0 3 2
";

    #[test]
    fn test_part1() {
        let mut solution = Solution::new(INPUT).unwrap();
        let answer = solution.part1().unwrap().to_string();
        assert_eq!(answer, "2");
    }
}
