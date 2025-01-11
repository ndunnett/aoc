#[derive(Clone)]
struct Game {
    red: u32,
    green: u32,
    blue: u32,
}

impl TryFrom<&str> for Game {
    type Error = Error;

    fn try_from(line: &str) -> Result<Self, Self::Error> {
        let (_, line) = line
            .split_once(':')
            .ok_or(anyhow!("failed to split line"))?;

        let mut game = Self::new();

        for (num, col) in line.split_whitespace().tuples() {
            match col.chars().next() {
                Some('r') => game.red = game.red.max(num.parse()?),
                Some('g') => game.green = game.green.max(num.parse()?),
                Some('b') => game.blue = game.blue.max(num.parse()?),
                _ => return Err(anyhow!("failed to parse as colour: {col}")),
            }
        }

        Ok(game)
    }
}

impl Game {
    fn new() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0,
        }
    }
}

#[derive(Clone)]
struct Solution {
    games: Vec<Game>,
}

impl Solver for Solution {
    fn new(input: &str) -> Anyhow<Self> {
        Ok(Self {
            games: input
                .lines()
                .map(Game::try_from)
                .collect::<Anyhow<Vec<_>>>()?,
        })
    }

    fn part1(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self.games.iter().enumerate().fold(0, |acc, (i, game)| {
            if game.red > 12 || game.green > 13 || game.blue > 14 {
                acc
            } else {
                acc + i + 1
            }
        }))
    }

    fn part2(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self
            .games
            .iter()
            .fold(0, |acc, game| acc + game.red * game.green * game.blue))
    }
}

aoc::solution!();

#[cfg(test)]
mod test {
    use super::{Solution, Solver};

    const INPUT: &str = r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_part1() {
        let mut solution = Solution::new(INPUT).unwrap();
        let answer = solution.part1().unwrap().to_string();
        assert_eq!(answer, "8");
    }

    #[test]
    fn test_part2() {
        let mut solution = Solution::new(INPUT).unwrap();
        let answer = solution.part2().unwrap().to_string();
        assert_eq!(answer, "2286");
    }
}
