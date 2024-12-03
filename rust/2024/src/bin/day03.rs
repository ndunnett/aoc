type Expr = (u32, u32);

struct Parser<'a> {
    chars: std::iter::Peekable<std::str::Chars<'a>>,
    dont: bool,
    finish: bool,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars().peekable(),
            dont: false,
            finish: false,
        }
    }

    fn match_pattern(&mut self, pattern: &[char]) -> bool {
        if self.chars.clone().zip(pattern).all(|(a, &b)| a == b) {
            for _ in 0..pattern.len() {
                self.chars.next();
            }

            true
        } else {
            false
        }
    }

    fn parse(mut self) -> (Vec<Expr>, Vec<Expr>) {
        let mut do_exprs = Vec::new();
        let mut dont_exprs = Vec::new();

        while !self.finish {
            if let Some(expr) = self.expr() {
                if self.dont {
                    dont_exprs.push(expr);
                } else {
                    do_exprs.push(expr);
                }
            }
        }

        (do_exprs, dont_exprs)
    }

    fn expr(&mut self) -> Option<Expr> {
        match self.chars.next() {
            Some('d') => self.do_(),
            Some('m') => self.mul(),
            None => {
                self.finish = true;
                None
            }
            _ => None,
        }
    }

    fn do_(&mut self) -> Option<Expr> {
        if self.match_pattern(&['o', 'n', '\'', 't', '(', ')']) {
            self.dont = true;
        } else if self.match_pattern(&['o', '(', ')']) {
            self.dont = false;
        }

        None
    }

    fn mul(&mut self) -> Option<Expr> {
        if self.match_pattern(&['u', 'l']) {
            self.chars.next_if_eq(&'(')?;
            let a = self.number()?;
            self.chars.next_if_eq(&',')?;
            let b = self.number()?;
            self.chars.next_if_eq(&')')?;
            Some((a, b))
        } else {
            None
        }
    }

    fn number(&mut self) -> Option<u32> {
        let mut num = Vec::new();

        while let Some(c) = self.chars.next_if(|&c| c.is_ascii_digit()) {
            num.push(c as u32 - b'0' as u32);
        }

        if !num.is_empty() {
            Some(num.iter().enumerate().fold(0, |acc, (i, n)| {
                acc + n * 10_u32.pow((num.len() - 1 - i) as u32)
            }))
        } else {
            None
        }
    }
}

struct Solution {
    do_exprs: Vec<Expr>,
    dont_exprs: Vec<Expr>,
}

impl Solver for Solution {
    fn new(input: &str) -> Anyhow<Self> {
        let (do_exprs, dont_exprs) = Parser::new(input).parse();

        Ok(Self {
            do_exprs,
            dont_exprs,
        })
    }

    fn part1(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self
            .do_exprs
            .iter()
            .chain(self.dont_exprs.iter())
            .fold(0, |acc, (a, b)| acc + a * b))
    }

    fn part2(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self.do_exprs.iter().fold(0, |acc, (a, b)| acc + a * b))
    }
}

aoc::solution!();

#[cfg(test)]
mod test {
    use super::{Solution, Solver};

    const INPUT1: &str = r"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const INPUT2: &str =
        r"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5)))";

    #[test]
    fn test_part1() {
        let mut solution = Solution::new(INPUT1).unwrap();
        let answer = solution.part1().unwrap().to_string();
        assert_eq!(answer, "161");
    }

    #[test]
    fn test_part2() {
        let mut solution = Solution::new(INPUT2).unwrap();
        let answer = solution.part2().unwrap().to_string();
        assert_eq!(answer, "48");
    }
}
