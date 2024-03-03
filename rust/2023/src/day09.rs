use itertools::Itertools;

fn parse(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .collect()
}

fn decompose(mut components: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let last = components.last().unwrap();

    if last.iter().all_equal() {
        return components;
    }

    components.push(last.windows(2).map(|p| p[1] - p[0]).collect_vec());
    decompose(components)
}

fn predict(history: &[i32]) -> i32 {
    decompose(vec![history.to_vec()])
        .iter()
        .map(|c| c.last().unwrap())
        .sum()
}

fn part1(input: &str) -> i32 {
    parse(input).iter().map(|h| predict(h)).sum()
}

fn part2(input: &str) -> i32 {
    parse(input)
        .iter()
        .map(|h| predict(&h.iter().copied().rev().collect_vec()))
        .sum()
}

#[allow(dead_code)]
pub fn puzzle() {
    let input = crate::input::load_input(9);
    println!("part 1 = {}", part1(&input));
    println!("part 2 = {}", part2(&input));
}

#[cfg(test)]
mod test {
    use super::{part1, part2};

    const INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 114);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 2);
    }
}
