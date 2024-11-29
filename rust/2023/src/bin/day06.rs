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

fn part2_parse(line: &str) -> usize {
    line.split(':')
        .last()
        .unwrap()
        .replace(' ', "")
        .parse()
        .unwrap()
}

pub struct Solution {
    lines: Vec<String>,
}

impl Solver for Solution {
    fn new(input: &str) -> Anyhow<Self> {
        Ok(Self {
            lines: input.lines().map(String::from).collect(),
        })
    }

    fn part1(&mut self) -> Anyhow<impl fmt::Display> {
        let times = part1_parse(&self.lines[0]);
        let distances = part1_parse(&self.lines[1]);

        Ok(times
            .iter()
            .zip(distances.iter())
            .fold(1, |acc, (&time, &distance)| {
                acc * count_winners(time, distance)
            }))
    }

    fn part2(&mut self) -> Anyhow<impl fmt::Display> {
        let time = part2_parse(&self.lines[0]);
        let distance = part2_parse(&self.lines[1]);

        Ok(count_winners(time, distance))
    }
}

aoc::solution!();

#[cfg(test)]
mod test {
    use super::{Solution, Solver};

    const INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_part1() {
        let mut solution = Solution::new(INPUT).unwrap();
        let answer = solution.part1().unwrap().to_string();
        assert_eq!(answer, "288");
    }

    #[test]
    fn test_part2() {
        let mut solution = Solution::new(INPUT).unwrap();
        let answer = solution.part2().unwrap().to_string();
        assert_eq!(answer, "71503");
    }
}
