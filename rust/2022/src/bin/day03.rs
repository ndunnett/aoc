fn get_priority(char: char) -> i32 {
    if char.is_lowercase() {
        char as i32 - 'a' as i32 + 1
    } else {
        char as i32 - 'A' as i32 + 27
    }
}

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
        let mut priorities = 0;

        for line in self.input.lines() {
            let compartments = line.split_at(line.len() / 2);

            for char in compartments.0.chars() {
                if compartments.1.contains(char) {
                    priorities += get_priority(char);
                    break;
                }
            }
        }

        Ok(priorities)
    }

    fn part2(&mut self) -> Anyhow<impl fmt::Display> {
        let mut priorities = 0;

        for i in (0..self.input.lines().count()).step_by(3) {
            let line_1 = self
                .input
                .lines()
                .nth(i)
                .ok_or(anyhow!("ran out of lines"))?;

            let line_2 = self
                .input
                .lines()
                .nth(i + 1)
                .ok_or(anyhow!("ran out of lines"))?;

            let line_3 = self
                .input
                .lines()
                .nth(i + 2)
                .ok_or(anyhow!("ran out of lines"))?;

            for char in line_1.chars() {
                if line_2.contains(char) && line_3.contains(char) {
                    priorities += get_priority(char);
                    break;
                }
            }
        }

        Ok(priorities)
    }
}

aoc::solution!();
