aoc::solution!();

fn solve(input: &str) -> Vec<i32> {
    let mut reindeer = Vec::new();
    let mut current = 0;

    for line in input.lines() {
        if line.is_empty() {
            reindeer.push(current);
            current = 0;
        } else {
            current += line.trim().parse::<i32>().unwrap();
        }
    }

    reindeer
}

fn part1(input: &str) -> i32 {
    *solve(input).iter().max().unwrap()
}

fn part2(input: &str) -> i32 {
    solve(input)[0..3].iter().sum()
}
