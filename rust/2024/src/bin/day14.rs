#[derive(Clone)]
struct Point {
    x: i16,
    y: i16,
}

impl Point {
    fn new(x: i16, y: i16) -> Self {
        Self { x, y }
    }
}

#[derive(Clone)]
struct Robot {
    p: Point,
    v: Point,
}

#[derive(Clone)]
struct Solution {
    robots: Vec<Robot>,
    width: i16,
    height: i16,
}

impl Solver for Solution {
    fn new(input: &str) -> Anyhow<Self> {
        Ok(Self {
            width: 101,
            height: 103,
            robots: NumberParserSigned::from(input)
                .tuples()
                .map(|(px, py, vx, vy)| Robot {
                    p: Point::new(px, py),
                    v: Point::new(vx, vy),
                })
                .collect(),
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
        // divide the room to get the middle thirds
        let ha = self.height / 3;
        let hb = self.height - ha;
        let wa = self.width / 3;
        let wb = self.width - wa;

        // approximate entropy in the middle of the room after 100 seconds until it reaches a predetermined threshold
        // the density of the robots should be much higher when forming a pattern than when randomly distributed
        //
        // expected robot count in the middle when randomly distributed:
        //   e = mid_positions * p
        //   => mid_positions = width // 3 * height // 3 = 101 // 3 * 103 // 3 = 1122
        //   => p = robot_count / (width * height) = 500 / (101 * 103) = 0.048
        //   => e = 1122 * 0.048 = 53.856
        //
        // so when the robot count in the middle is significantly higher than 53.856, it signifies that there must be a pattern
        (100..(self.width as i32 * self.height as i32))
            .into_par_iter()
            .find_any(|t| {
                self.robots
                    .iter()
                    .filter(|r| {
                        let (x, y) = (
                            (r.p.x as i32 + r.v.x as i32 * t).rem_euclid(self.width as i32) as i16,
                            (r.p.y as i32 + r.v.y as i32 * t).rem_euclid(self.height as i32) as i16,
                        );

                        x > wa && x < wb && y > ha && y < hb
                    })
                    .count()
                    >= 150
            })
            .ok_or(anyhow!("failed to solve part 2"))
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
