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
        let steps = (self.width * self.height) as i32;
        let p = self.robots.len() as f64 / (self.width * self.height) as f64;
        let avg_variance = (1.0 - p) / (p * p);

        let count = 2 * self.robots.len() as u32;
        let variance_count_threshold = (0.75 * 2.0_f64.sqrt() * avg_variance) as u32 * count;

        (100..steps)
            .into_par_iter()
            .find_any(|step| {
                let (mut sum, mut sum_sq) = (0, 0);

                for robot in &self.robots {
                    let x = (robot.p.x as i32 + robot.v.x as i32 * step)
                        .rem_euclid(self.width as i32) as u32;
                    let y = (robot.p.y as i32 + robot.v.y as i32 * step)
                        .rem_euclid(self.height as i32) as u32;
                    sum += x + y;
                    sum_sq += x * x + y * y;
                }

                // count is factored into variance to reduce mathematical operations required:
                //   mean = sum / count
                //   variance = sum_sq / count - mean^2
                //   variance = sum_sq / count - sum^2 / count^2
                //   variance * count = sum_sq - sum^2 / count

                let variance_count = sum_sq - sum * sum / count;
                variance_count < variance_count_threshold
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
