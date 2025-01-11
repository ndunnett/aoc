#[derive(Clone)]
struct Solution {
    input: String,
}

impl Solver for Solution {
    fn new(input: &str) -> Anyhow<Self> {
        Ok(Self {
            input: String::from(input),
        })
    }

    fn part1(&mut self) -> Anyhow<impl fmt::Display> {
        let mut answer = 0;

        for line in self.input.lines() {
            let pairs: Vec<&str> = line.split(',').collect();

            let a: Vec<i32> = pairs[0]
                .split('-')
                .map(|x| x.parse::<i32>().unwrap())
                .collect();

            let b: Vec<i32> = pairs[1]
                .split('-')
                .map(|x| x.parse::<i32>().unwrap())
                .collect();

            if (a[0] >= b[0] && a[1] <= b[1]) || (a[0] <= b[0] && a[1] >= b[1]) {
                answer += 1;
            }
        }

        Ok(answer)
    }

    fn part2(&mut self) -> Anyhow<impl fmt::Display> {
        let mut answer = 0;

        for line in self.input.lines() {
            let pairs: Vec<&str> = line.split(',').collect();

            let a: Vec<i32> = pairs[0]
                .split('-')
                .map(|x| x.parse::<i32>().unwrap())
                .collect();

            let b: Vec<i32> = pairs[1]
                .split('-')
                .map(|x| x.parse::<i32>().unwrap())
                .collect();

            if (a[0] <= b[0] || a[1] <= b[1] || a[0] <= b[1])
                && (a[1] >= b[0] || a[0] <= b[1])
                && (a[1] >= b[0] || a[0] >= b[0])
            {
                answer += 1;
            }
        }

        Ok(answer)
    }
}

aoc::solution!();
