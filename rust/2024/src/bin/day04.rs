#[derive(Clone, Copy, PartialEq)]
enum Letter {
    X,
    M,
    A,
    S,
}

impl TryFrom<char> for Letter {
    type Error = Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'X' => Ok(Self::X),
            'M' => Ok(Self::M),
            'A' => Ok(Self::A),
            'S' => Ok(Self::S),
            _ => Err(anyhow!("failed to parse char: {c}")),
        }
    }
}

struct Grid {
    letters: Vec<Letter>,
    size: usize,
}

impl From<&str> for Grid {
    fn from(input: &str) -> Self {
        let letters = input
            .chars()
            .filter_map(|c| Letter::try_from(c).ok())
            .collect::<Vec<_>>();

        let size = letters.len().isqrt();

        Self { letters, size }
    }
}

type Index<'a> = Box<dyn Iterator<Item = (usize, usize, usize, usize)> + 'a>;

impl Grid {
    fn iter(&self) -> GridIterator<'_> {
        GridIterator {
            grid: self,
            index: self.horizontal_index(),
            method: GridIterationMethod::Horizontal,
        }
    }

    fn horizontal_index(&self) -> Index<'_> {
        Box::new((0..self.size - 3).flat_map(move |x| {
            (0..self.size).map(move |y| {
                (
                    x + y * self.size,
                    x + 1 + y * self.size,
                    x + 2 + y * self.size,
                    x + 3 + y * self.size,
                )
            })
        }))
    }

    fn vertical_index(&self) -> Index<'_> {
        Box::new((0..self.size).flat_map(move |x| {
            (0..self.size - 3).map(move |y| {
                (
                    x + y * self.size,
                    x + (y + 1) * self.size,
                    x + (y + 2) * self.size,
                    x + (y + 3) * self.size,
                )
            })
        }))
    }

    fn diagonal_up_index(&self) -> Index<'_> {
        Box::new((0..self.size - 3).flat_map(move |x| {
            (3..self.size).map(move |y| {
                (
                    x + y * self.size,
                    x + 1 + (y - 1) * self.size,
                    x + 2 + (y - 2) * self.size,
                    x + 3 + (y - 3) * self.size,
                )
            })
        }))
    }

    fn diagonal_down_index(&self) -> Index<'_> {
        Box::new((0..self.size - 3).flat_map(move |x| {
            (0..self.size - 3).map(move |y| {
                (
                    x + y * self.size,
                    x + 1 + (y + 1) * self.size,
                    x + 2 + (y + 2) * self.size,
                    x + 3 + (y + 3) * self.size,
                )
            })
        }))
    }

    fn cross_index(&self) -> Index<'_> {
        Box::new((0..self.size - 2).flat_map(move |x| {
            (0..self.size - 2).filter_map(move |y| {
                if self.letters[x + 1 + (y + 1) * self.size] == Letter::A {
                    Some((
                        x + y * self.size,
                        x + 2 + y * self.size,
                        x + 2 + (y + 2) * self.size,
                        x + (y + 2) * self.size,
                    ))
                } else {
                    None
                }
            })
        }))
    }
}

enum GridIterationMethod {
    Horizontal,
    Vertical,
    DiagonalUp,
    DiagonalDown,
    Cross,
}

struct GridIterator<'a> {
    grid: &'a Grid,
    index: Index<'a>,
    method: GridIterationMethod,
}

impl GridIterator<'_> {
    fn cross(mut self) -> Self {
        self.method = GridIterationMethod::Cross;
        self.index = self.grid.cross_index();
        self
    }
}

impl Iterator for GridIterator<'_> {
    type Item = (Letter, Letter, Letter, Letter);

    fn next(&mut self) -> Option<Self::Item> {
        let mut next = self.index.next();

        if next.is_none() {
            match self.method {
                GridIterationMethod::Horizontal => {
                    self.method = GridIterationMethod::Vertical;
                    self.index = self.grid.vertical_index();
                }
                GridIterationMethod::Vertical => {
                    self.method = GridIterationMethod::DiagonalUp;
                    self.index = self.grid.diagonal_up_index();
                }
                GridIterationMethod::DiagonalUp => {
                    self.method = GridIterationMethod::DiagonalDown;
                    self.index = self.grid.diagonal_down_index();
                }
                GridIterationMethod::DiagonalDown | GridIterationMethod::Cross => {
                    return None;
                }
            }

            next = self.index.next();
        }

        if let Some((a, b, c, d)) = next {
            Some((
                self.grid.letters[a],
                self.grid.letters[b],
                self.grid.letters[c],
                self.grid.letters[d],
            ))
        } else {
            None
        }
    }
}

struct Solution {
    grid: Grid,
}

impl Solver for Solution {
    fn new(input: &str) -> Anyhow<Self> {
        Ok(Self {
            grid: Grid::from(input),
        })
    }

    fn part1(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self
            .grid
            .iter()
            .filter(|tup| {
                matches!(
                    tup,
                    (Letter::X, Letter::M, Letter::A, Letter::S)
                        | (Letter::S, Letter::A, Letter::M, Letter::X)
                )
            })
            .count())
    }

    fn part2(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self
            .grid
            .iter()
            .cross()
            .filter(|tup| {
                matches!(
                    tup,
                    (Letter::M, Letter::M, Letter::S, Letter::S)
                        | (Letter::M, Letter::S, Letter::S, Letter::M)
                        | (Letter::S, Letter::S, Letter::M, Letter::M)
                        | (Letter::S, Letter::M, Letter::M, Letter::S)
                )
            })
            .count())
    }
}

aoc::solution!();

#[cfg(test)]
mod test {
    use super::{Solution, Solver};

    const INPUT: &str = r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn test_part1() {
        let mut solution = Solution::new(INPUT).unwrap();
        let answer = solution.part1().unwrap().to_string();
        assert_eq!(answer, "18");
    }

    #[test]
    fn test_part2() {
        let mut solution = Solution::new(INPUT).unwrap();
        let answer = solution.part2().unwrap().to_string();
        assert_eq!(answer, "9");
    }
}
