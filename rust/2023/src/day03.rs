use regex::Regex;
use std::collections::HashSet;

#[allow(dead_code)]
fn part1(input: &str) -> u32 {
    let mut sum = 0;
    let symbol_re = Regex::new(r"[^\d.]").unwrap();
    let number_re = Regex::new(r"\d+").unwrap();

    let symbols_vec: Vec<Vec<usize>> = input
        .lines()
        .map(|line| symbol_re.find_iter(line).map(|m| m.start()).collect())
        .collect();

    for (i, line) in input.lines().enumerate() {
        sum += number_re
            .find_iter(line)
            .filter(|m| {
                symbols_vec
                    .iter()
                    .skip(i.saturating_sub(1))
                    .take(3)
                    .any(|symbols| {
                        symbols
                            .iter()
                            .any(|&pos| (m.start().saturating_sub(1)..=m.end()).contains(&pos))
                    })
            })
            .map(|m| m.as_str().parse::<u32>().unwrap())
            .sum::<u32>();
    }

    sum
}

fn get_edges(i: usize, j: usize) -> [(usize, usize); 8] {
    [
        (i.saturating_sub(1), j.saturating_sub(1)),
        (i.saturating_sub(1), j),
        (i.saturating_sub(1), j + 1),
        (i, j.saturating_sub(1)),
        (i, j + 1),
        (i + 1, j.saturating_sub(1)),
        (i + 1, j),
        (i + 1, j + 1),
    ]
}

#[allow(dead_code)]
fn part2(input: &str) -> u32 {
    let mut sum = 0;
    let gear_re = Regex::new(r"\*").unwrap();
    let number_re = Regex::new(r"\d+").unwrap();

    let numbers_vec = input
        .lines()
        .map(|line| {
            number_re
                .find_iter(line)
                .map(|m| (m.range(), m.as_str().parse::<u32>().unwrap()))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    for (i, line) in input.lines().enumerate() {
        for m in gear_re.find_iter(line) {
            let edges = get_edges(i, m.start());
            let gear_numbers = edges
                .iter()
                .flat_map(|(x, y)| {
                    numbers_vec.get(*x).and_then(|vec| {
                        vec.iter()
                            .find(|(range, _)| *y >= range.start && *y < range.end)
                            .map(|(_, number)| *number)
                    })
                })
                .collect::<HashSet<_>>();

            if gear_numbers.len() == 2 {
                sum += gear_numbers.iter().product::<u32>();
            }
        }
    }

    sum
}

#[allow(dead_code)]
pub fn puzzle() {
    let input = crate::input::load_input(3);
    println!("part 1 = {}", part1(&input));
    println!("part 2 = {}", part2(&input));
}
