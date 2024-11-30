use itertools::FoldWhile::{Continue, Done};

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        a %= b;
        std::mem::swap(&mut a, &mut b);
    }

    a
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

struct Solution {
    instructions: Vec<char>,
    nodes: HashMap<String, (String, String)>,
}

impl Solver for Solution {
    fn new(input: &str) -> Anyhow<Self> {
        let mut lines = input.lines().filter(|line| !line.is_empty());
        let instructions = lines.next().unwrap().chars().collect();

        let f = |s: &str| {
            s.chars()
                .filter(|c| c.is_alphanumeric())
                .collect::<String>()
        };

        let nodes = lines.fold(HashMap::new(), |mut nodes, line| {
            if let Some((key, values)) = line.split_once('=') {
                if let Some((left, right)) = values.split_once(',') {
                    nodes.insert(f(key), (f(left), f(right)));
                }
            }
            nodes
        });

        Ok(Self {
            instructions,
            nodes,
        })
    }

    fn part1(&mut self) -> Anyhow<impl fmt::Display> {
        let mut location = String::from("AAA");

        Ok(self
            .instructions
            .iter()
            .cycle()
            .fold_while(0, |steps, &instruction| {
                let paths = self.nodes.get(&location).unwrap();

                location = match instruction {
                    'L' => paths.0.clone(),
                    'R' => paths.1.clone(),
                    _ => panic!("invalid instruction: {instruction}"),
                };

                if location == "ZZZ" {
                    Done(steps + 1)
                } else {
                    Continue(steps + 1)
                }
            })
            .into_inner())
    }

    fn part2(&mut self) -> Anyhow<impl fmt::Display> {
        let locations = self
            .nodes
            .keys()
            .filter(|&node| node.ends_with('A'))
            .cloned()
            .collect_vec();

        let cycles = locations.iter().map(|start| {
            let mut location = start.clone();

            self.instructions
                .iter()
                .cycle()
                .fold_while(0, |steps, &instruction| {
                    let paths = self.nodes.get(&location).unwrap();

                    location = match instruction {
                        'L' => paths.0.clone(),
                        'R' => paths.1.clone(),
                        _ => panic!("invalid instruction: {}", instruction),
                    };

                    if location.ends_with('Z') {
                        Done(steps + 1)
                    } else {
                        Continue(steps + 1)
                    }
                })
                .into_inner()
        });

        Ok(cycles.reduce(lcm).unwrap())
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
