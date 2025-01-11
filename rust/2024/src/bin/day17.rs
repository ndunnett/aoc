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

    fn get_value(&self, a: usize, b: usize, c: usize) -> usize {
        match self {
            Self::Literal(n) => *n as usize,
            Self::A => a,
            Self::B => b,
            Self::C => c,
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

impl TryFrom<(u8, u8)> for Operation {
    type Error = Error;

    fn try_from(value: (u8, u8)) -> Result<Self, Self::Error> {
        match value {
            (0, operand) => Ok(Self {
                opcode: Opcode::Adv,
                operand: Operand::parse_combo(operand)?,
            }),
            (1, operand) => Ok(Self {
                opcode: Opcode::Bxl,
                operand: Operand::Literal(operand),
            }),
            (2, operand) => Ok(Self {
                opcode: Opcode::Bst,
                operand: Operand::parse_combo(operand)?,
            }),
            (3, operand) => Ok(Self {
                opcode: Opcode::Jnz,
                operand: Operand::Literal(operand),
            }),
            (4, operand) => Ok(Self {
                opcode: Opcode::Bxc,
                operand: Operand::Literal(operand),
            }),
            (5, operand) => Ok(Self {
                opcode: Opcode::Out,
                operand: Operand::parse_combo(operand)?,
            }),
            (6, operand) => Ok(Self {
                opcode: Opcode::Bdv,
                operand: Operand::parse_combo(operand)?,
            }),
            (7, operand) => Ok(Self {
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

#[derive(Clone)]
struct Solution {
    program: Vec<u8>,
    operations: Vec<Operation>,
    a: usize,
}

impl Solution {
    fn compute(&self, mut a: usize) -> Anyhow<Vec<u8>> {
        let mut b = 0;
        let mut c = 0;
        let mut output = Vec::new();
        let mut pointer = 0;

        while let Some(Operation { opcode, operand }) = self.operations.get(pointer) {
            match (opcode, operand) {
                (Opcode::Adv, _) => a = a / 2_usize.pow(operand.get_value(a, b, c) as u32),
                (Opcode::Bxl, Operand::Literal(n)) => b ^= *n as usize,
                (Opcode::Bst, _) => b = operand.get_value(a, b, c) % 8,
                (Opcode::Jnz, Operand::Literal(n)) => {
                    if a != 0 {
                        pointer = *n as usize / 2;
                        continue;
                    }
                }
                (Opcode::Bxc, _) => b ^= c,
                (Opcode::Out, _) => output.push((operand.get_value(a, b, c) % 8) as u8),
                (Opcode::Bdv, _) => b = a / 2_usize.pow(operand.get_value(a, b, c) as u32),
                (Opcode::Cdv, _) => c = a / 2_usize.pow(operand.get_value(a, b, c) as u32),
                _ => return Err(anyhow!("invalid operation")),
            }

            pointer += 1;
        }

        Ok(output)
    }
}

impl Solver for Solution {
    fn new(input: &str) -> Anyhow<Self> {
        let (registers, program) = input
            .split_once("\n\n")
            .ok_or(anyhow!("failed to split input"))?;

        let a = registers
            .split([' ', '\n'])
            .nth(2)
            .ok_or(anyhow!("failed to parse registers"))?
            .parse::<usize>()?;

        let program = program
            .split_once(' ')
            .ok_or(anyhow!("failed to split program"))?
            .1
            .bytes()
            .step_by(2)
            .map(|b| b - b'0')
            .collect::<Vec<_>>();

        let operations = program
            .iter()
            .cloned()
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
        Ok(self
            .compute(self.a)?
            .into_iter()
            .map(|b| char::from(b + b'0'))
            .join(","))
    }

    fn part2(&mut self) -> Anyhow<impl fmt::Display> {
        let mut queue = vec![(self.program.len() - 1, 0)];

        while let Some((offset, a)) = queue.pop() {
            for i in 0..8 {
                if self.compute(a * 8 + i)?[..] == self.program[offset..] {
                    if offset == 0 {
                        return Ok(a * 8 + i);
                    }

                    queue.insert(0, (offset - 1, a * 8 + i));
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
