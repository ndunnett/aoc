#[derive(Clone)]
struct Solution {
    rotations: Vec<i32>,
}

impl Solver for Solution {
    fn new(input: &str) -> Anyhow<Self> {
        let bytes = input.as_bytes();
        let mut rotations = Vec::with_capacity(4500);

        unsafe {
            let mut p = bytes.as_ptr();
            let end = p.add(bytes.len());

            while p < end {
                let negative = *p == b'L';
                p = p.add(1);

                let mut n = (*p - b'0') as i32;
                p = p.add(1);

                for _ in 0..3 {
                    let byte = *p;
                    p = p.add(1);

                    if byte < b'0' {
                        break;
                    }

                    n = n * 10 + (byte - b'0') as i32;
                }

                rotations.push(if negative { -n } else { n });
            }
        }

        Ok(Self { rotations })
    }

    fn part1(&mut self) -> Anyhow<impl fmt::Display> {
        let mut position = 50;
        let mut count = 0;

        for rotation in &self.rotations {
            position = (position + rotation) % 100;

            if position == 0 {
                count += 1;
            }
        }

        Ok(count)
    }

    fn part2(&mut self) -> Anyhow<impl fmt::Display> {
        let mut position = 50;
        let mut count = 0;

        for &rotation in &self.rotations {
            if rotation >= 0 {
                position += rotation;
                count += position / 100;
                position %= 100;
            } else {
                if position == 0 {
                    count += rotation / -100;
                } else if -rotation >= position {
                    count += ((position + rotation) / -100) + 1;
                }

                position = (position + rotation + 10000) % 100;
            }
        }

        Ok(count)
    }
}

aoc::solution!();

#[cfg(test)]
mod test {
    use super::{Solution, Solver};

    const INPUT: &str = r"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
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
        assert_eq!(answer, "6");
    }
}
