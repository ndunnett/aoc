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
    fn from_tuple(value: (i64, i64, i64, i64, i64, i64)) -> Self {
        Self {
            ax: value.0,
            ay: value.1,
            bx: value.2,
            by: value.3,
            px: value.4,
            py: value.5,
        }
    }

    fn solve<const N: i64>(&self) -> Option<i64> {
        let px = self.px + N;
        let py = self.py + N;

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
        let di = px * self.by - py * self.bx;
        let dj = py * self.ax - px * self.ay;

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
            machines: NumberParser::from(input)
                .tuples()
                .map(Machine::from_tuple)
                .collect(),
        })
    }

    fn part1(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self
            .machines
            .iter()
            .filter_map(Machine::solve::<0>)
            .sum::<i64>())
    }

    fn part2(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self
            .machines
            .iter()
            .filter_map(Machine::solve::<10000000000000>)
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
