#[derive(Clone)]
struct Machine {
    ax: i64,
    ay: i64,
    bx: i64,
    by: i64,
    px: i64,
    py: i64,
}

impl Machine {
    const PATTERN: [char; 4] = ['+', ',', '=', '\n'];
}

impl TryFrom<&str> for Machine {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Some((ax, ay, bx, by, px, py)) = value
            .split(&Self::PATTERN)
            .skip(1)
            .step_by(2)
            .map(|s| s.parse::<i64>().expect("failed to parse number"))
            .next_tuple()
        {
            Ok(Self {
                ax,
                ay,
                bx,
                by,
                px,
                py,
            })
        } else {
            Err(anyhow!("failed to parse machine"))
        }
    }
}

impl Machine {
    fn solve(&self) -> Option<i64> {
        // px = i*ax + j*bx
        // py = i*ay + j*by
        //
        // A = ⌈ax bx⌉  X = ⌈i⌉  AX = C = ⌈px⌉
        //     ⌊ay by⌋      ⌊j⌋           ⌊py⌋
        //
        // D = |A| = ax*by - ay*bx
        // Di = px*by - py*bx
        // Dj = py*ax - px*ay
        //
        // i = Di / D
        // j = Dj / D
        // answer = 3*i + j

        let d = self.ax * self.by - self.ay * self.bx;
        let di = self.px * self.by - self.py * self.bx;
        let dj = self.py * self.ax - self.px * self.ay;

        if di % d == 0 && dj % d == 0 {
            Some(3 * di / d + dj / d)
        } else {
            None
        }
    }
}

#[derive(Clone)]
struct Solution {
    machines: Vec<Machine>,
}

impl Solver for Solution {
    fn new(input: &str) -> Anyhow<Self> {
        Ok(Self {
            machines: input
                .split("\n\n")
                .map(Machine::try_from)
                .collect::<Anyhow<Vec<_>>>()?,
        })
    }

    fn part1(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self.machines.iter().filter_map(Machine::solve).sum::<i64>())
    }

    fn part2(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self
            .machines
            .iter()
            .filter_map(|m| {
                Machine {
                    ax: m.ax,
                    ay: m.ay,
                    bx: m.bx,
                    by: m.by,
                    px: m.px + 10000000000000,
                    py: m.py + 10000000000000,
                }
                .solve()
            })
            .sum::<i64>())
    }
}

aoc::solution!();

#[cfg(test)]
mod test {
    use super::{Solution, Solver};

    const INPUT: &str = r"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    fn test_part1() {
        let mut solution = Solution::new(INPUT).unwrap();
        let answer = solution.part1().unwrap().to_string();
        assert_eq!(answer, "480");
    }

    #[test]
    fn test_part2() {
        let mut solution = Solution::new(INPUT).unwrap();
        let answer = solution.part2().unwrap().to_string();
        assert_eq!(answer, "875318608908");
    }
}
