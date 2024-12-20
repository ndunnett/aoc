fn inspect_report(report: &[u8]) -> bool {
    if report[0] < report[report.len() - 1] {
        report.windows(2).all(|v| v[0] < v[1] && v[1] - v[0] <= 3)
    } else {
        report.windows(2).all(|v| v[0] > v[1] && v[0] - v[1] <= 3)
    }
}

fn atoi(chars: &[char]) -> u8 {
    let mut acc = 0;

    for i in 0..chars.len() {
        let mut n = chars[chars.len() - 1 - i] as u8 - b'0';

        for _ in 0..i {
            n = (n << 3) + (n << 1);
        }

        acc += n;
    }

    acc
}

struct Solution {
    reports: Vec<Vec<u8>>,
}

impl Solver for Solution {
    fn new(input: &str) -> Anyhow<Self> {
        let mut reports = vec![Vec::new()];
        reports.reserve(1000);

        let mut level = Vec::new();

        for c in input.chars() {
            let i = reports.len() - 1;

            match c {
                c if c.is_ascii_digit() => level.push(c),
                ' ' => {
                    reports[i].push(atoi(&level));
                    level.clear();
                }
                '\n' => {
                    reports[i].push(atoi(&level));
                    level.clear();
                    reports.push(Vec::new());
                }
                _ => return Err(anyhow!("invalid character when parsing: '{c:?}'")),
            }
        }

        reports.pop();
        Ok(Self { reports })
    }

    fn part1(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self
            .reports
            .par_iter()
            .filter(|report| inspect_report(report))
            .count())
    }

    fn part2(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self
            .reports
            .par_iter()
            .filter(|report| {
                (0..report.len()).any(|i| {
                    inspect_report(&[&report[0..i], &report[i + 1..report.len()]].concat())
                })
            })
            .count())
    }
}

aoc::solution!();

#[cfg(test)]
mod test {
    use super::{Solution, Solver};

    // actual input has a trailing new line
    const INPUT: &str = r"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

    #[test]
    fn test_part1() {
        let mut solution = Solution::new(INPUT).unwrap();
        let answer = solution.part1().unwrap().to_string();
        assert_eq!(answer, "2");
    }

    #[test]
    fn test_part2() {
        let mut solution = Solution::new(INPUT).unwrap();
        let answer = solution.part2().unwrap().to_string();
        assert_eq!(answer, "4");
    }
}
