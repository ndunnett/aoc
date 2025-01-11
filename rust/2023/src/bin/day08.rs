const AAA: Element = Element::const_from(&['A', 'A', 'A']);
const ZZZ: Element = Element::const_from(&['Z', 'Z', 'Z']);

fn lcm(a: usize, b: usize) -> usize {
    let mut gcd = a;
    let mut den = b;

    while den != 0 {
        gcd %= den;
        std::mem::swap(&mut gcd, &mut den);
    }

    a * b / gcd
}

#[derive(Clone)]
enum Instruction {
    Left,
    Right,
}

impl From<char> for Instruction {
    fn from(value: char) -> Self {
        match value {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => panic!("failed to parse instruction"),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct Element(u32);

impl Element {
    const fn const_from(value: &[char]) -> Self {
        Self((value[0] as u32) << 16 | (value[1] as u32) << 8 | value[2] as u32)
    }

    fn ends_with(&self, char: char) -> bool {
        self.0 as u8 == char as u8
    }
}

#[derive(Clone)]
struct Solution {
    instructions: Vec<Instruction>,
    nodes: HashMap<Element, (Element, Element)>,
}

impl Solver for Solution {
    fn new(input: &str) -> Anyhow<Self> {
        let mut lines = input.lines().filter(|line| !line.is_empty());

        let instructions = lines
            .next()
            .ok_or(anyhow!("failed to parse instructions line"))?
            .chars()
            .map(Instruction::from)
            .collect();

        let nodes = lines
            .map(|line| {
                let chars = line.chars().collect_vec();

                (
                    Element::const_from(&chars[0..3]),
                    (
                        Element::const_from(&chars[7..10]),
                        Element::const_from(&chars[12..15]),
                    ),
                )
            })
            .collect();

        Ok(Self {
            instructions,
            nodes,
        })
    }

    fn part1(&mut self) -> Anyhow<impl fmt::Display> {
        let mut location = &AAA;

        for (i, instruction) in self.instructions.iter().cycle().enumerate() {
            if let Some((left, right)) = self.nodes.get(location) {
                location = match instruction {
                    Instruction::Left => left,
                    Instruction::Right => right,
                };

                if location == &ZZZ {
                    return Ok(i + 1);
                }
            } else {
                return Err(anyhow!("failed to retrieve node"));
            }
        }

        Err(anyhow!("failed to find path"))
    }

    fn part2(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self
            .nodes
            .keys()
            .par_bridge()
            .filter_map(|node| {
                if !node.ends_with('A') {
                    return None;
                }

                let mut location = node;

                for (i, instruction) in self.instructions.iter().cycle().enumerate() {
                    if let Some((left, right)) = self.nodes.get(location) {
                        location = match instruction {
                            Instruction::Left => left,
                            Instruction::Right => right,
                        };

                        if location.ends_with('Z') {
                            return Some(i + 1);
                        }
                    } else {
                        return None;
                    }
                }

                None
            })
            .reduce(|| 1, lcm))
    }
}

aoc::solution!();

#[cfg(test)]
mod test {
    use super::{Solution, Solver};

    const INPUT1: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    const INPUT2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    const INPUT3: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn test_part1_1() {
        let mut solution = Solution::new(INPUT1).unwrap();
        let answer = solution.part1().unwrap().to_string();
        assert_eq!(answer, "2");
    }

    #[test]
    fn test_part1_2() {
        let mut solution = Solution::new(INPUT2).unwrap();
        let answer = solution.part1().unwrap().to_string();
        assert_eq!(answer, "6");
    }

    #[test]
    fn test_part2() {
        let mut solution = Solution::new(INPUT3).unwrap();
        let answer = solution.part2().unwrap().to_string();
        assert_eq!(answer, "6");
    }
}
