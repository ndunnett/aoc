#[derive(Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Clone)]
struct Robot {
    p: Point,
    v: Point,
}

impl Robot {
    const SPLIT_PATTERN: [char; 3] = ['=', ',', ' '];
}

impl TryFrom<&str> for Robot {
    type Error = Error;

    fn try_from(line: &str) -> Result<Self, Self::Error> {
        if let Some((_, px, py, _, vx, vy)) = line.split(&Self::SPLIT_PATTERN).next_tuple() {
            Ok(Self {
                p: Point::new(px.parse::<i32>()?, py.parse::<i32>()?),
                v: Point::new(vx.parse::<i32>()?, vy.parse::<i32>()?),
            })
        } else {
            Err(anyhow!("failed to parse robot"))
        }
    }
}

#[derive(Clone)]
struct Solution {
    robots: Vec<Robot>,
    width: i32,
    height: i32,
}

impl Solver for Solution {
    fn new(input: &str) -> Anyhow<Self> {
        Ok(Self {
            width: 101,
            height: 103,
            robots: input
                .lines()
                .map(Robot::try_from)
                .collect::<Anyhow<Vec<_>>>()?,
        })
    }

    fn part1(&mut self) -> Anyhow<impl fmt::Display> {
        let quad_width = self.width / 2;
        let quad_height = self.height / 2;

        Ok(self
            .robots
            .iter()
            .fold([0; 4], |mut quads, robot| {
                let point = (
                    (robot.p.x + robot.v.x * 100).rem_euclid(self.width),
                    (robot.p.y + robot.v.y * 100).rem_euclid(self.height),
                );

                match point {
                    (x, y) if x < quad_width && y < quad_height => quads[0] += 1,
                    (x, y) if x > quad_width && y < quad_height => quads[1] += 1,
                    (x, y) if x < quad_width && y > quad_height => quads[2] += 1,
                    (x, y) if x > quad_width && y > quad_height => quads[3] += 1,
                    _ => {}
                }

                quads
            })
            .iter()
            .product::<usize>())
    }

    fn part2(&mut self) -> Anyhow<impl fmt::Display> {
        (100..(self.width * self.height))
            .into_par_iter()
            .find_first(|t| {
                let mut grid = vec![0_u128; self.height as usize];

                for robot in &self.robots {
                    let (x, y) = (
                        (robot.p.x + robot.v.x * t).rem_euclid(self.width) as usize,
                        (robot.p.y + robot.v.y * t).rem_euclid(self.height) as usize,
                    );

                    grid[y] |= 1 << x;
                }

                // looking to see 2 rows with a continous line of at least 16 robots
                // this will match the lines above and below the christmas tree
                grid.iter()
                    .filter(|&row| {
                        (0..self.width)
                            .step_by(16)
                            .map(|x| 0xFFFF << x)
                            .any(|mask| row & mask == mask)
                    })
                    .take(2)
                    .count()
                    >= 2
            })
            .ok_or(anyhow!("failed to solve"))
    }
}

aoc::solution!();

#[cfg(test)]
mod test {
    use super::{Solution, Solver};

    const INPUT: &str = r"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    #[test]
    fn test_part1() {
        let mut solution = Solution::new(INPUT).unwrap();
        solution.width = 11;
        solution.height = 7;
        let answer = solution.part1().unwrap().to_string();
        assert_eq!(answer, "12");
    }

    // Look for a pattern like this in part 2:
    //
    // ###############################
    // #                             #
    // #                             #
    // #                             #
    // #                             #
    // #              #              #
    // #             ###             #
    // #            #####            #
    // #           #######           #
    // #          #########          #
    // #            #####            #
    // #           #######           #
    // #          #########          #
    // #         ###########         #
    // #        #############        #
    // #          #########          #
    // #         ###########         #
    // #        #############        #
    // #       ###############       #
    // #      #################      #
    // #        #############        #
    // #       ###############       #
    // #      #################      #
    // #     ###################     #
    // #    #####################    #
    // #             ###             #
    // #             ###             #
    // #             ###             #
    // #                             #
    // #                             #
    // #                             #
    // #                             #
    // ###############################
}
