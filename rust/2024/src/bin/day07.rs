#[derive(Clone)]
struct Equation {
    value: u64,
    operands: [u16; 12],
    len: u8,
}

impl Equation {
    fn new(value: u64) -> Self {
        Self {
            value,
            operands: [0; 12],
            len: 0,
        }
    }

    fn push_operand(&mut self, n: u64) {
        self.operands[11 - self.len as usize] = n as u16;
        self.len += 1;
    }

    fn slice(&self) -> &[u16] {
        &self.operands[12 - self.len as usize..12]
    }
}

#[derive(Clone)]
struct Solution {
    equations: Vec<Equation>,
}

impl Solver for Solution {
    fn new(input: &str) -> Anyhow<Self> {
        Ok(Self {
            equations: input
                .lines()
                .map(|line| {
                    let mut parser = NumberParser::from(line);
                    let value = parser.next().expect("failed to parse equation");
                    let mut equation = Equation::new(value);

                    for n in parser {
                        equation.push_operand(n);
                    }

                    equation
                })
                .collect(),
        })
    }

    fn part1(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self
            .equations
            .par_iter()
            .filter_map(|e| {
                let mut queue = Vec::with_capacity(16);
                queue.push((e.value, e.slice()));

                while let Some((s, n)) = queue.pop() {
                    if n.len() == 1 && s as u16 == n[0] {
                        return Some(e.value);
                    } else if n.len() > 1 {
                        if s > n[0] as u64 {
                            queue.push((s - n[0] as u64, &n[1..]));
                        }

                        if s % n[0] as u64 == 0 {
                            queue.push((s / n[0] as u64, &n[1..]));
                        }
                    }
                }

                None
            })
            .sum::<u64>())
    }

    fn part2(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self
            .equations
            .par_iter()
            .filter_map(|e| {
                let mut queue = Vec::with_capacity(16);
                queue.push((e.value, e.slice()));

                while let Some((s, n)) = queue.pop() {
                    if n.len() == 1 && s as u16 == n[0] {
                        return Some(e.value);
                    } else if n.len() > 1 {
                        if s > n[0] as u64 {
                            queue.push((s - n[0] as u64, &n[1..]));
                        }

                        if s % n[0] as u64 == 0 {
                            queue.push((s / n[0] as u64, &n[1..]));
                        }

                        let partition = 10_u64.pow(n[0].ilog10() + 1);

                        if s > partition && s % partition == n[0] as u64 {
                            queue.push((s / partition, &n[1..]));
                        }
                    }
                }

                None
            })
            .sum::<u64>())
    }
}

aoc::solution!();

#[cfg(test)]
mod test {
    use super::{Solution, Solver};

    const INPUT: &str = r"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_part1() {
        let mut solution = Solution::new(INPUT).unwrap();
        let answer = solution.part1().unwrap().to_string();
        assert_eq!(answer, "3749");
    }

    #[test]
    fn test_part2() {
        let mut solution = Solution::new(INPUT).unwrap();
        let answer = solution.part2().unwrap().to_string();
        assert_eq!(answer, "11387");
    }
}
