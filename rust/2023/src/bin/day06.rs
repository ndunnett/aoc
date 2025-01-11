fn quadratic(time: &f64, distance: &f64) -> f64 {
    let disc = (time.powf(2.0) - 4.0 * distance).sqrt();
    let root1 = (time + disc) / 2.0;
    let root2 = (time - disc) / 2.0;
    root1.ceil() - root2.floor() - 1.0
}

fn parse_as_list(line: &str) -> Anyhow<Vec<f64>> {
    Ok(line
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse())
        .collect::<ParseFloatResult<_>>()?)
}

fn parse_as_number(line: &str) -> Anyhow<f64> {
    Ok(line
        .split(':')
        .last()
        .ok_or(anyhow!("failed to extract numbers from line"))?
        .replace(' ', "")
        .parse()?)
}

#[derive(Clone)]
struct Solution {
    lines: (String, String),
}

impl Solver for Solution {
    fn new(input: &str) -> Anyhow<Self> {
        let mut lines = input.lines().map(String::from);

        Ok(Self {
            lines: (
                lines
                    .next()
                    .ok_or(anyhow!("failed to extract first line"))?,
                lines
                    .next()
                    .ok_or(anyhow!("failed to extract second line"))?,
            ),
        })
    }

    fn part1(&mut self) -> Anyhow<impl fmt::Display> {
        let times = parse_as_list(&self.lines.0)?;
        let distances = parse_as_list(&self.lines.1)?;

        Ok(times
            .iter()
            .zip(distances.iter())
            .fold(1.0, |acc, (time, distance)| acc * quadratic(time, distance)))
    }

    fn part2(&mut self) -> Anyhow<impl fmt::Display> {
        let time = parse_as_number(&self.lines.0)?;
        let distance = parse_as_number(&self.lines.1)?;

        Ok(quadratic(&time, &distance))
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
