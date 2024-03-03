use regex::Regex;
use std::collections::HashMap;

#[allow(dead_code)]
fn part1(input: &str) -> u32 {
    let re = Regex::new(r"\d+").unwrap();

    input.lines().fold(0, |acc, line| {
        let mut numbers = line.splitn(3, |c| c == ':' || c == '|').skip(1).map(|s| {
            re.find_iter(s)
                .map(|m| m.as_str().parse::<u32>().unwrap())
                .collect()
        });

        let winners = numbers.next().unwrap();
        let picks: Vec<_> = numbers.last().unwrap();
        let matches = winners
            .iter()
            .filter(|winner| picks.contains(winner))
            .count() as u32;

        if matches > 0 {
            acc + 2_u32.pow(matches - 1)
        } else {
            acc
        }
    })
}

#[allow(dead_code)]
fn part2(input: &str) -> u32 {
    let re = Regex::new(r"\d+").unwrap();
    let mut copy_counts: HashMap<usize, u32> = HashMap::new();

    input.lines().enumerate().fold(0, |acc, (i, line)| {
        let mut numbers = line.splitn(3, |c| c == ':' || c == '|').skip(1).map(|s| {
            re.find_iter(s)
                .map(|m| m.as_str().parse::<u32>().unwrap())
                .collect()
        });

        let winners = numbers.next().unwrap();
        let picks: Vec<_> = numbers.last().unwrap();
        let matches = winners
            .iter()
            .filter(|winner| picks.contains(winner))
            .count();

        let n = copy_counts.get(&i).unwrap_or(&0) + 1;

        if matches > 0 {
            for j in 1..=matches {
                copy_counts
                    .entry(i + j)
                    .and_modify(|count| *count += n)
                    .or_insert(n);
            }
        }

        acc + n
    })
}

#[allow(dead_code)]
pub fn puzzle() {
    let input = crate::input::load_input(4);
    println!("part 1 = {}", part1(&input));
    println!("part 2 = {}", part2(&input));
}
