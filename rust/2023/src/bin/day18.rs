#[derive(Clone, Copy)]
enum Direction {
    R,
    D,
    L,
    U,
}

impl Direction {
    #[inline(always)]
    const fn delta(self) -> (i64, i64) {
        match self {
            Self::R => (1, 0),
            Self::D => (0, 1),
            Self::L => (-1, 0),
            Self::U => (0, -1),
        }
    }
}

type Instruction = (Direction, i64);

#[derive(Clone)]
struct Step {
    direction: Direction,
    dec: u8,
    hex: u32,
}

impl Step {
    #[inline(always)]
    fn decimal_instruction(&self) -> Instruction {
        (self.direction, self.dec as i64)
    }

    #[inline(always)]
    fn hex_instruction(&self) -> Instruction {
        let direction = match self.hex & 0xF {
            0 => Direction::R,
            1 => Direction::D,
            2 => Direction::L,
            3 => Direction::U,
            n => unreachable!("invalid hex value for Direction: {n}"),
        };

        (direction, (self.hex >> 4) as i64)
    }
}

const DEC_VALUE: [u8; 256] = {
    let mut dec_value = [255; 256];
    let mut index = b'0';

    while index <= b'9' {
        dec_value[index as usize] = index - b'0';
        index += 1;
    }

    dec_value
};

const HEX_VALUE: [u8; 256] = {
    let mut hex_value = [255; 256];
    let mut index = b'0';

    while index <= b'9' {
        hex_value[index as usize] = index - b'0';
        index += 1;
    }

    index = b'a';

    while index <= b'f' {
        hex_value[index as usize] = index - b'a' + 10;
        index += 1;
    }

    hex_value
};

#[derive(Clone)]
struct Solution {
    plan: Vec<Step>,
}

impl Solution {
    #[inline(always)]
    fn solve<F: Fn(&Step) -> Instruction>(&self, parse_step: F) -> i64 {
        let mut perimeter = 0;
        let mut inner_area = 0;
        let (mut ax, mut ay) = (0, 0);
        let (mut bx, mut by) = (0, 0);

        for step in &self.plan {
            let (direction, magnitude) = parse_step(step);
            perimeter += magnitude;
            let (dx, dy) = direction.delta();
            ax += dx * magnitude;
            ay += dy * magnitude;
            inner_area += bx * ay - ax * by;
            (bx, by) = (ax, ay);
        }

        ((inner_area + perimeter) >> 1) + 1
    }
}

impl Solver for Solution {
    fn new(input: &str) -> Anyhow<Self> {
        let bytes = input.as_bytes();
        let mut index = 0;
        let mut plan = Vec::with_capacity(input.len() / 10);

        while index < bytes.len() {
            // The first byte is always the direction.
            let direction = match bytes[index] {
                b'R' => Direction::R,
                b'D' => Direction::D,
                b'L' => Direction::L,
                b'U' => Direction::U,
                _ => unsafe { std::hint::unreachable_unchecked() },
            };

            // Skip 2 bytes to get to the decimal value.
            index += 2;

            // Parse the 1-2 digit decimal value.
            let mut dec = DEC_VALUE[bytes[index] as usize];
            index += 1;

            if bytes[index].is_ascii_digit() {
                dec = (dec << 1) + (dec << 3) + DEC_VALUE[bytes[index] as usize];
                index += 1;
            }

            // Skip 3 bytes to get to the hex value.
            index += 3;

            // Parse the 6 digit hex value.
            let mut hex = 0;

            for i in 0..6 {
                hex = (hex << 4) | HEX_VALUE[bytes[index + i] as usize] as u32;
            }

            // Move past the hex value and then skip 2 bytes to start the next loop iteration on the first byte of the next line.
            index += 6 + 2;

            plan.push(Step {
                direction,
                dec,
                hex,
            });
        }

        Ok(Self { plan })
    }

    fn part1(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self.solve(Step::decimal_instruction))
    }

    fn part2(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self.solve(Step::hex_instruction))
    }
}

aoc::solution!();

#[cfg(test)]
mod test {
    use super::{Solution, Solver};

    const INPUT: &str = r"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    #[test]
    fn test_part1() {
        let mut solution = Solution::new(INPUT).unwrap();
        let answer = solution.part1().unwrap().to_string();
        assert_eq!(answer, "62");
    }

    #[test]
    fn test_part2() {
        let mut solution = Solution::new(INPUT).unwrap();
        let answer = solution.part2().unwrap().to_string();
        assert_eq!(answer, "952408144115");
    }
}
