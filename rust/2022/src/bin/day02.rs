enum Moves {
    Rock,
    Paper,
    Scissors,
}

struct Solution {
    lines: Vec<String>,
}

impl Solver for Solution {
    fn new(input: &str) -> Anyhow<Self> {
        Ok(Self {
            lines: input.lines().map(String::from).collect(),
        })
    }

    fn part1(&mut self) -> Anyhow<impl fmt::Display> {
        let mut score = 0;
        let mut opp_move: Moves;
        let mut my_move: Moves;

        for line in self.lines.iter() {
            match line.chars().next().ok_or(anyhow!("ran out of chars"))? {
                'A' => opp_move = Moves::Rock,
                'B' => opp_move = Moves::Paper,
                'C' => opp_move = Moves::Scissors,
                _ => return Err(anyhow!("failed to match opp_move")),
            }

            match line.chars().nth(2).ok_or(anyhow!("ran out of chars"))? {
                'X' => my_move = Moves::Rock,
                'Y' => my_move = Moves::Paper,
                'Z' => my_move = Moves::Scissors,
                _ => return Err(anyhow!("failed to match my_move")),
            }

            match my_move {
                Moves::Rock => match opp_move {
                    Moves::Rock => score += 4,
                    Moves::Paper => score += 1,
                    Moves::Scissors => score += 7,
                },
                Moves::Paper => match opp_move {
                    Moves::Rock => score += 8,
                    Moves::Paper => score += 5,
                    Moves::Scissors => score += 2,
                },
                Moves::Scissors => match opp_move {
                    Moves::Rock => score += 3,
                    Moves::Paper => score += 9,
                    Moves::Scissors => score += 6,
                },
            }
        }

        Ok(score)
    }

    fn part2(&mut self) -> Anyhow<impl fmt::Display> {
        let mut score = 0;
        let mut opp_move: Moves;

        for line in self.lines.iter() {
            match line.chars().next().ok_or(anyhow!("ran out of chars"))? {
                'A' => opp_move = Moves::Rock,
                'B' => opp_move = Moves::Paper,
                'C' => opp_move = Moves::Scissors,
                _ => return Err(anyhow!("failed to match opp_move")),
            }

            match line.chars().nth(2).ok_or(anyhow!("ran out of chars"))? {
                'X' => match opp_move {
                    Moves::Rock => score += 3,
                    Moves::Paper => score += 1,
                    Moves::Scissors => score += 2,
                },
                'Y' => match opp_move {
                    Moves::Rock => score += 4,
                    Moves::Paper => score += 5,
                    Moves::Scissors => score += 6,
                },
                'Z' => match opp_move {
                    Moves::Rock => score += 8,
                    Moves::Paper => score += 9,
                    Moves::Scissors => score += 7,
                },
                _ => return Err(anyhow!("failed to match my_move")),
            }
        }

        Ok(score)
    }
}

aoc::solution!();
