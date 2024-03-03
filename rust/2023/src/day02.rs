use regex::Regex;
use std::cmp::max;

#[allow(dead_code)]
fn part1(input: &str) -> u32 {
    let mut sum = 0;
    let game_re = Regex::new(r"Game (?P<num>\d+):").unwrap();
    let exprs = vec![
        (Regex::new(r"(?P<num>\d+) red").unwrap(), 12),
        (Regex::new(r"(?P<num>\d+) green").unwrap(), 13),
        (Regex::new(r"(?P<num>\d+) blue").unwrap(), 14),
    ];

    for line in input.lines() {
        let game = game_re.captures(line).unwrap().name("num").unwrap().as_str().parse::<u32>().unwrap();
        let mut possible = true;

        'outer: for (re, cubes) in &exprs {
            for cap in re.captures_iter(line) {
                if cap.name("num").unwrap().as_str().parse::<u32>().unwrap() > *cubes {
                    possible = false;
                    break 'outer;
                }
            }
        }

        if possible {
            sum += game;
        }
    }

    sum
}

#[allow(dead_code)]
fn part2(input: &str) -> u32 {
    let mut sum = 0;
    let exprs = vec![
        Regex::new(r"(?P<num>\d+) red").unwrap(),
        Regex::new(r"(?P<num>\d+) green").unwrap(),
        Regex::new(r"(?P<num>\d+) blue").unwrap(),
    ];

    for line in input.lines() {
        let mut power = 1;

        for re in &exprs {
            let mut cubes = 0;

            for cap in re.captures_iter(line) {
                cubes = max(cubes, cap.name("num").unwrap().as_str().parse::<u32>().unwrap());
            }

            power *= cubes;
        }

        sum += power;
    }

    sum
}

#[allow(dead_code)]
pub fn puzzle() {
    let input = crate::input::load_input(2);
    println!("part 1 = {}", part1(&input));
    println!("part 2 = {}", part2(&input));
}
