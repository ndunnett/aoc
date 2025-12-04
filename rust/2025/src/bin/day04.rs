fn solve<const REMOVE: bool>(map: &mut Vec<Vec<bool>>) -> usize {
    let mut count = 0;
    let height = map.len();
    let width = map[0].len();

    loop {
        let mut _map = map.clone();
        let mut removed = 0;

        for y in 0..height {
            for x in 0..width {
                if !map[y][x] {
                    continue;
                }

                let mut neighbours = 0;
                let not_far_left = x > 0;
                let not_far_right = x < width - 1;
                let not_far_up = y > 0;
                let not_far_down = y < height - 1;

                if not_far_up && map[y - 1][x] {
                    neighbours += 1;
                }

                if not_far_up && not_far_left && map[y - 1][x - 1] {
                    neighbours += 1;
                }

                if not_far_up && not_far_right && map[y - 1][x + 1] {
                    neighbours += 1;
                }

                if not_far_down && map[y + 1][x] {
                    neighbours += 1;
                }

                if not_far_down && not_far_left && map[y + 1][x - 1] {
                    neighbours += 1;
                }

                if not_far_down && not_far_right && map[y + 1][x + 1] {
                    neighbours += 1;
                }

                if not_far_left && map[y][x - 1] {
                    neighbours += 1;
                }

                if not_far_right && map[y][x + 1] {
                    neighbours += 1;
                }

                if neighbours < 4 {
                    if REMOVE {
                        _map[y][x] = false;
                        removed += 1;
                    }

                    count += 1;
                }
            }
        }

        if REMOVE {
            if removed == 0 {
                break;
            }

            *map = _map;
        } else {
            break;
        }
    }

    count
}

#[derive(Clone)]
struct Solution {
    map: Vec<Vec<bool>>,
}

impl Solver for Solution {
    fn new(input: &str) -> Anyhow<Self> {
        Ok(Self {
            map: input
                .lines()
                .map(|line| line.bytes().map(|byte| byte == b'@').collect())
                .collect(),
        })
    }

    fn part1(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(solve::<false>(&mut self.map))
    }

    fn part2(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(solve::<true>(&mut self.map))
    }
}

aoc::solution!();

#[cfg(test)]
mod test {
    use super::{Solution, Solver};

    const INPUT: &str = r"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";

    #[test]
    fn test_part1() {
        let mut solution = Solution::new(INPUT).unwrap();
        let answer = solution.part1().unwrap().to_string();
        assert_eq!(answer, "13");
    }

    #[test]
    fn test_part2() {
        let mut solution = Solution::new(INPUT).unwrap();
        let answer = solution.part2().unwrap().to_string();
        assert_eq!(answer, "43");
    }
}
