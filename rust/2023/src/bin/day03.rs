#[derive(Clone)]
struct Symbol {
    value: char,
    row: usize,
    col: usize,
}

#[derive(Clone)]
struct Number {
    value: u32,
    top: usize,
    bottom: usize,
    left: usize,
    right: usize,
}

impl Number {
    fn collision(&self, symbol: &Symbol) -> bool {
        self.left <= symbol.col
            && symbol.col <= self.right
            && self.top <= symbol.row
            && symbol.row <= self.bottom
    }
}

#[derive(Clone)]
struct Solution {
    numbers: Vec<Number>,
    symbols: Vec<Symbol>,
}

impl Solver for Solution {
    fn new(input: &str) -> Anyhow<Self> {
        let mut numbers = Vec::new();
        let mut symbols = Vec::new();
        let mut chars = input.chars().peekable();
        let mut row: usize = 1;
        let mut column: usize = 1;

        while let Some(next) = chars.next() {
            column += 1;

            match next {
                '\n' => {
                    row += 1;
                    column = 1;
                }
                c if c.is_ascii_digit() => {
                    let mut val = vec![c];
                    let start = column - 2;

                    while let Some(peek) = chars.peek() {
                        if peek.is_ascii_digit() {
                            val.push(*peek);
                            chars.next();
                            column += 1;
                        } else {
                            numbers.push(Number {
                                value: val.into_iter().collect::<String>().parse()?,
                                top: row - 1,
                                bottom: row + 1,
                                left: start,
                                right: column,
                            });
                            break;
                        }
                    }
                }
                '-' | '@' | '*' | '+' | '$' | '#' | '&' | '=' | '%' | '/' => {
                    symbols.push(Symbol {
                        value: next,
                        row,
                        col: column - 1,
                    });
                }
                _ => {}
            }
        }

        Ok(Self { numbers, symbols })
    }

    fn part1(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self
            .numbers
            .par_iter()
            .filter_map(|number| {
                for symbol in &self.symbols {
                    if number.collision(symbol) {
                        return Some(number.value);
                    }
                }

                None
            })
            .sum::<u32>())
    }

    fn part2(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self
            .symbols
            .par_iter()
            .filter(|symbol| symbol.value == '*')
            .filter_map(|gear| {
                let touching = self
                    .numbers
                    .iter()
                    .filter(|number| number.collision(gear))
                    .take(2)
                    .collect_tuple();

                if let Some((a, b)) = touching {
                    Some(a.value * b.value)
                } else {
                    None
                }
            })
            .sum::<u32>())
    }
}

aoc::solution!();

#[cfg(test)]
mod test {
    use super::{Solution, Solver};

    const INPUT: &str = r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn test_part1() {
        let mut solution = Solution::new(INPUT).unwrap();
        let answer = solution.part1().unwrap().to_string();
        assert_eq!(answer, "4361");
    }

    #[test]
    fn test_part2() {
        let mut solution = Solution::new(INPUT).unwrap();
        let answer = solution.part2().unwrap().to_string();
        assert_eq!(answer, "467835");
    }
}
