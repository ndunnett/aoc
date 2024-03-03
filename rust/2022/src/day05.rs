use regex::Regex;
use std::collections::HashMap;

#[allow(dead_code)]
pub fn puzzle() {
    let input = crate::input::load_input(5);
    let mut answer = Vec::new();

    fn parse_stacks(input: &str) -> HashMap<i32, Vec<char>> {
        let mut stacks = HashMap::new();

        for i in (1..34).step_by(4) {
            let stack_number = input
                .lines()
                .nth(8)
                .unwrap()
                .chars()
                .nth(i)
                .unwrap()
                .to_digit(10)
                .unwrap() as i32;
            stacks.insert(stack_number, Vec::new());

            for j in (0..8).rev() {
                let value = input.lines().nth(j).unwrap().chars().nth(i).unwrap();

                if value.is_alphabetic() {
                    stacks.get_mut(&stack_number).unwrap().push(value);
                }
            }
        }

        stacks
    }

    fn parse_command(line: &str) -> (i32, i32, i32) {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"(?:move\s)(?P<n>[\d]+)(?:\sfrom\s)(?P<src>[\d]+)(?:\sto\s)(?P<dest>[\d]+)"
            )
            .unwrap();
        }

        let caps = RE.captures(line).unwrap();
        (
            caps["n"].parse::<i32>().unwrap(),
            caps["src"].parse::<i32>().unwrap(),
            caps["dest"].parse::<i32>().unwrap(),
        )
    }

    let mut stacks = parse_stacks(&input);

    for line in input.lines().skip(10) {
        let (n, src, dest) = parse_command(line);
        let mut crates = Vec::new();

        for _ in 0..n {
            crates.push(stacks.get_mut(&src).unwrap().pop().unwrap());
        }

        stacks.get_mut(&dest).unwrap().append(&mut crates);
    }

    for i in 1..=9 {
        answer.push(*stacks[&i].last().unwrap());
    }

    println!("Part 1 answer: {}", String::from_iter(answer.iter()));

    answer = Vec::new();
    let mut stacks = parse_stacks(&input);

    for line in input.lines().skip(10) {
        let (n, src, dest) = parse_command(line);
        let mut crates = Vec::new();

        for _ in 0..n {
            crates.push(stacks.get_mut(&src).unwrap().pop().unwrap());
        }

        crates.reverse();
        stacks.get_mut(&dest).unwrap().append(&mut crates);
    }

    for i in 1..=9 {
        answer.push(*stacks[&i].last().unwrap());
    }

    println!("Part 2 answer: {}", String::from_iter(answer.iter()));
}
