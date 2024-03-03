use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use std::collections::HashMap;

fn parse(input: &str) -> (Vec<char>, HashMap<String, (String, String)>) {
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

    (instructions, nodes)
}

fn part1(input: &str) -> u64 {
    let (instructions, nodes) = parse(input);
    let mut location = String::from("AAA");

    instructions
        .iter()
        .cycle()
        .fold_while(0, |steps, &instruction| {
            let paths = nodes.get(&location).unwrap();

            location = match instruction {
                'L' => paths.0.clone(),
                'R' => paths.1.clone(),
                _ => panic!("invalid instruction: {}", instruction),
            };

            if location == "ZZZ" {
                Done(steps + 1)
            } else {
                Continue(steps + 1)
            }
        })
        .into_inner()
}

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

#[allow(dead_code)]
fn part2(input: &str) -> u64 {
    let (instructions, nodes) = parse(input);

    let locations = nodes
        .keys()
        .filter(|&node| node.ends_with('A'))
        .cloned()
        .collect_vec();

    let cycles = locations.iter().map(|start| {
        let mut location = start.clone();

        instructions
            .iter()
            .cycle()
            .fold_while(0, |steps, &instruction| {
                let paths = nodes.get(&location).unwrap();

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

    cycles.reduce(lcm).unwrap()
}

#[allow(dead_code)]
pub fn puzzle() {
    let input = crate::input::load_input(8);
    println!("part 1 = {}", part1(&input));
    println!("part 2 = {}", part2(&input));
}

#[cfg(test)]
mod test {
    use super::{part1, part2};

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
    fn test_part1() {
        assert_eq!(part1(INPUT1), 2);
        assert_eq!(part1(INPUT2), 6);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT3), 6);
    }
}
