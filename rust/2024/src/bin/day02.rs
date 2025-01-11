fn inspect_report(report: &[u8]) -> bool {
    if report[0] < report[report.len() - 1] {
        report.windows(2).all(|v| v[0] < v[1] && v[1] - v[0] <= 3)
    } else {
        report.windows(2).all(|v| v[0] > v[1] && v[0] - v[1] <= 3)
    }
}

#[derive(Clone)]
struct Solution {
    reports: Vec<Vec<u8>>,
}

impl Solver for Solution {
    fn new(input: &str) -> Anyhow<Self> {
        let mut reports = Vec::with_capacity(input.lines().count());
        let mut report = Vec::new();
        let mut bytes = input.bytes();

        while let Some(b) = bytes.next() {
            let mut num = b - b'0';

            if let Some(mut b) = bytes.next() {
                if b.is_ascii_digit() {
                    num = (num << 1) + (num << 3) + b - b'0';
                    b = bytes.next().unwrap();
                }

                report.push(num);

                if b == b'\n' {
                    reports.push(report.clone());
                    report.clear();
                }
            }
        }

        Ok(Self { reports })
    }

    fn part1(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self
            .reports
            .iter()
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
