fn count_winners(time: usize, distance: usize) -> usize {
    (1..time)
        .filter(|speed| (time - speed) * speed > distance)
        .count()
}

fn part1_parse(line: &str) -> Vec<usize> {
    line.split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect()
}

#[allow(dead_code)]
fn part1(input: &str) -> usize {
    let lines: Vec<&str> = input.lines().collect();
    let times = part1_parse(lines[0]);
    let distances = part1_parse(lines[1]);

    times
        .iter()
        .zip(distances.iter())
        .fold(1, |acc, (&time, &distance)| {
            acc * count_winners(time, distance)
        })
}

fn part2_parse(line: &str) -> usize {
    line.split(':')
        .last()
        .unwrap()
        .replace(' ', "")
        .parse()
        .unwrap()
}

#[allow(dead_code)]
fn part2(input: &str) -> usize {
    let lines: Vec<&str> = input.lines().collect();
    let time = part2_parse(lines[0]);
    let distance = part2_parse(lines[1]);

    count_winners(time, distance)
}

#[allow(dead_code)]
pub fn puzzle() {
    let input = crate::input::load_input(6);
    println!("part 1 = {}", part1(&input));
    println!("part 2 = {}", part2(&input));
}

#[cfg(test)]
mod test {
    use super::{part1, part2};

    const INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 288);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 71503);
    }
}
