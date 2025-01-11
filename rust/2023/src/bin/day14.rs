#[derive(Clone)]
struct Solution {
    round: Vec<Vec<bool>>,
    square: Vec<Vec<bool>>,
}

impl Solution {
    fn tilt_north(&mut self) {
        for y in 1..self.round.len() {
            for x in 0..self.round[0].len() {
                if self.round[y][x] {
                    let mut target = None;

                    for candidate in (0..y).rev() {
                        if !self.round[candidate][x] && !self.square[candidate][x] {
                            target = Some(candidate);
                        } else {
                            break;
                        }
                    }

                    if let Some(target) = target {
                        self.round[y][x] = false;
                        self.round[target][x] = true;
                    }
                }
            }
        }
    }

    fn tilt_west(&mut self) {
        for y in 0..self.round.len() {
            for x in 1..self.round[0].len() {
                if self.round[y][x] {
                    let mut target = None;

                    for candidate in (0..x).rev() {
                        if !self.round[y][candidate] && !self.square[y][candidate] {
                            target = Some(candidate);
                        } else {
                            break;
                        }
                    }

                    if let Some(target) = target {
                        self.round[y][x] = false;
                        self.round[y][target] = true;
                    }
                }
            }
        }
    }

    fn tilt_south(&mut self) {
        for y in (0..self.round.len() - 1).rev() {
            for x in 0..self.round[0].len() {
                if self.round[y][x] {
                    let mut target = None;

                    for candidate in y + 1..self.round.len() {
                        if !self.round[candidate][x] && !self.square[candidate][x] {
                            target = Some(candidate);
                        } else {
                            break;
                        }
                    }

                    if let Some(target) = target {
                        self.round[y][x] = false;
                        self.round[target][x] = true;
                    }
                }
            }
        }
    }

    fn tilt_east(&mut self) {
        for y in 0..self.round.len() {
            for x in (0..self.round[0].len() - 1).rev() {
                if self.round[y][x] {
                    let mut target = None;

                    for candidate in x + 1..self.round[0].len() {
                        if !self.round[y][candidate] && !self.square[y][candidate] {
                            target = Some(candidate);
                        } else {
                            break;
                        }
                    }

                    if let Some(target) = target {
                        self.round[y][x] = false;
                        self.round[y][target] = true;
                    }
                }
            }
        }
    }

    fn cycle(&mut self, n: usize) {
        let mut cache = FxHashMap::default();

        for i in 0..n {
            if let Some(j) = cache.get(&self.round) {
                if (n - i) % (i - j) == 0 {
                    return;
                }
            }

            cache.insert(self.round.clone(), i);

            self.tilt_north();
            self.tilt_west();
            self.tilt_south();
            self.tilt_east();
        }
    }

    fn weight(&self) -> usize {
        self.round.iter().enumerate().fold(0, |acc, (i, row)| {
            acc + row.iter().filter(|&&rock| rock).count() * (self.round.len() - i)
        })
    }
}

impl Solver for Solution {
    fn new(input: &str) -> Anyhow<Self> {
        let round = input
            .lines()
            .map(|line| line.chars().map(|c| c == 'O').collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let square = input
            .lines()
            .map(|line| line.chars().map(|c| c == '#').collect::<Vec<_>>())
            .collect::<Vec<_>>();

        Ok(Self { round, square })
    }

    fn part1(&mut self) -> Anyhow<impl fmt::Display> {
        self.tilt_north();
        Ok(self.weight())
    }

    fn part2(&mut self) -> Anyhow<impl fmt::Display> {
        self.cycle(1000000000);
        Ok(self.weight())
    }
}

aoc::solution!();

#[cfg(test)]
mod test {
    use super::{Solution, Solver};

    const INPUT: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn test_part1() {
        let mut solution = Solution::new(INPUT).unwrap();
        let answer = solution.part1().unwrap().to_string();
        assert_eq!(answer, "136");
    }

    #[test]
    fn test_part2() {
        let mut solution = Solution::new(INPUT).unwrap();
        let answer = solution.part2().unwrap().to_string();
        assert_eq!(answer, "64");
    }
}
