#[derive(Clone)]
enum Operand {
    Literal(u8),
    A,
    B,
    C,
}

impl Operand {
    fn parse_combo(value: u8) -> Anyhow<Self> {
        match value {
            0..=3 => Ok(Self::Literal(value)),
            4 => Ok(Self::A),
            5 => Ok(Self::B),
            6 => Ok(Self::C),
            _ => Err(anyhow!("failed to parse combo operand")),
        }
    }
}

#[derive(Clone)]
enum Opcode {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

#[derive(Clone)]
struct Operation {
    opcode: Opcode,
    operand: Operand,
}

impl TryFrom<(&u8, &u8)> for Operation {
    type Error = Error;

    fn try_from(value: (&u8, &u8)) -> Result<Self, Self::Error> {
        match value {
            (0, &operand) => Ok(Self {
                opcode: Opcode::Adv,
                operand: Operand::parse_combo(operand)?,
            }),
            (1, &operand) => Ok(Self {
                opcode: Opcode::Bxl,
                operand: Operand::Literal(operand),
            }),
            (2, &operand) => Ok(Self {
                opcode: Opcode::Bst,
                operand: Operand::parse_combo(operand)?,
            }),
            (3, &operand) => Ok(Self {
                opcode: Opcode::Jnz,
                operand: Operand::Literal(operand),
            }),
            (4, &operand) => Ok(Self {
                opcode: Opcode::Bxc,
                operand: Operand::Literal(operand),
            }),
            (5, &operand) => Ok(Self {
                opcode: Opcode::Out,
                operand: Operand::parse_combo(operand)?,
            }),
            (6, &operand) => Ok(Self {
                opcode: Opcode::Bdv,
                operand: Operand::parse_combo(operand)?,
            }),
            (7, &operand) => Ok(Self {
                opcode: Opcode::Cdv,
                operand: Operand::parse_combo(operand)?,
            }),
            _ => Err(anyhow!(
                "failed to parse operation: {}, {}",
                value.0,
                value.1
            )),
        }
    }
}

struct Computation<'a> {
    operations: &'a [Operation],
    pointer: usize,
    a: usize,
    b: usize,
    c: usize,
}

impl Computation<'_> {
    #[inline(always)]
    fn get_value(&self, operand: &Operand) -> usize {
        match operand {
            Operand::Literal(n) => *n as usize,
            Operand::A => self.a,
            Operand::B => self.b,
            Operand::C => self.c,
        }
    }
}

impl Iterator for Computation<'_> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        // Chronospatial Computer virtual machine implemented as an iterator
        // each iteration is an output value
        while let Some(Operation { opcode, operand }) = self.operations.get(self.pointer) {
            match (opcode, operand) {
                (Opcode::Adv, _) => self.a /= 2_usize.pow(self.get_value(operand) as u32),
                (Opcode::Bxl, Operand::Literal(n)) => self.b ^= *n as usize,
                (Opcode::Bst, _) => self.b = self.get_value(operand) % 8,
                (Opcode::Jnz, Operand::Literal(n)) => {
                    if self.a != 0 {
                        self.pointer = *n as usize / 2;
                        continue;
                    }
                }
                (Opcode::Bxc, _) => self.b ^= self.c,
                (Opcode::Out, _) => {
                    self.pointer += 1;
                    return Some((self.get_value(operand) % 8) as u8);
                }
                (Opcode::Bdv, _) => self.b = self.a / 2_usize.pow(self.get_value(operand) as u32),
                (Opcode::Cdv, _) => self.c = self.a / 2_usize.pow(self.get_value(operand) as u32),
                _ => unreachable!(),
            }

            self.pointer += 1;
        }

        None
    }
}

#[derive(Clone)]
struct Solution {
    program: Vec<u8>,
    operations: Vec<Operation>,
    a: usize,
}

impl Solution {
    fn compute(&self, a: usize) -> Computation<'_> {
        Computation {
            operations: &self.operations,
            pointer: 0,
            a,
            b: 0,
            c: 0,
        }
    }
}

impl Solver for Solution {
    fn new(input: &str) -> Anyhow<Self> {
        let mut parser = NumberParser::<usize>::from(input);

        let (a, _, _) = parser
            .next_tuple()
            .ok_or(anyhow!("failed to parse registers"))?;

        let program = parser.map(|n| n as u8).collect::<Vec<_>>();

        let operations = program
            .iter()
            .tuples::<(_, _)>()
            .map(Operation::try_from)
            .collect::<Anyhow<Vec<_>>>()?;

        Ok(Self {
            program,
            operations,
            a,
        })
    }

    fn part1(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self.compute(self.a).map(|b| char::from(b + b'0')).join(","))
    }

    fn part2(&mut self) -> Anyhow<impl fmt::Display> {
        let mut queue = Vec::with_capacity(32);
        queue.push((self.program.len() - 1, 0));

        // The program is an octal computer which mutates the last few bits and shifts register A on each scan. Quining
        // can be done one oct at a time, starting from the last output, by finding an offset to the current value of
        // register A to generate the correct output, then multiplying register A by 8 to move on to the next output.
        // Repeat until all outputs are matched for the given value of register A, using BFS to find the lowest value.
        while let Some((offset, a)) = queue.pop() {
            for candidate in a..a + 8 {
                if self
                    .compute(candidate)
                    .zip(self.program.iter().skip(offset))
                    .all(|(a, &b)| a == b)
                {
                    if offset == 0 {
                        return Ok(candidate);
                    }

                    queue.insert(0, (offset - 1, candidate * 8));
                }
            }
        }

        Err(anyhow!("failed to solve for a"))
    }
}

aoc::solution!();

#[cfg(test)]
mod test {
    use super::{Solution, Solver};

    const INPUT1: &str = r"Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    const INPUT2: &str = r"Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";

    #[test]
    fn test_part1() {
        let mut solution = Solution::new(INPUT1).unwrap();
        let answer = solution.part1().unwrap().to_string();
        assert_eq!(answer, "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_part2() {
        let mut solution = Solution::new(INPUT2).unwrap();
        let answer = solution.part2().unwrap().to_string();
        assert_eq!(answer, "117440");
    }
}
