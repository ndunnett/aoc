fn parse_numbers<S: AsRef<str>>(s: S) -> Anyhow<Vec<i64>> {
    s.as_ref()
        .split_whitespace()
        .map(|n| Ok(n.parse::<i64>()?))
        .collect::<Anyhow<Vec<_>>>()
}

#[derive(Clone)]
struct Layer {
    start: i64,
    end: i64,
    offset: i64,
}

impl TryFrom<&str> for Layer {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let numbers = parse_numbers(s)?;

        // [0] -> destination
        // [1] -> source
        // [2] -> length

        Ok(Self {
            start: numbers[1],
            end: numbers[1] + numbers[2],
            offset: numbers[0] - numbers[1],
        })
    }
}

#[derive(Clone)]
struct Solution {
    seeds: Vec<i64>,
    maps: Vec<Vec<Layer>>,
}

impl Solution {
    fn transform(&self, seed: i64) -> i64 {
        self.maps.iter().fold(seed, |acc, map| {
            map.iter()
                .find(|layer| layer.start <= acc && acc < layer.end)
                .map_or(acc, |layer| acc + layer.offset)
        })
    }
}

impl Solver for Solution {
    fn new(input: &str) -> Anyhow<Self> {
        let mut sections = input.split("\n\n").filter(|section| !section.is_empty());

        let seeds = parse_numbers(
            sections
                .next()
                .ok_or(anyhow!("empty input"))?
                .split_once(':')
                .ok_or(anyhow!("failed to split seeds"))?
                .1,
        )?;

        let maps = sections
            .map(|section| {
                section
                    .lines()
                    .skip(1)
                    .map(Layer::try_from)
                    .collect::<Anyhow<Vec<_>>>()
            })
            .collect::<Anyhow<Vec<_>>>()?;

        Ok(Self { seeds, maps })
    }

    fn part1(&mut self) -> Anyhow<impl fmt::Display> {
        self.seeds
            .iter()
            .map(|&seed| self.transform(seed))
            .min()
            .ok_or(anyhow!("no seeds"))
    }

    fn part2(&mut self) -> Anyhow<impl fmt::Display> {
        self.seeds
            .par_chunks_exact(2)
            .map(|seed_range| {
                let mut queue = vec![(seed_range[0], seed_range[1])];

                while let Some((start, len)) = queue.pop() {
                    if len < 15 {
                        return (0..len)
                            .map(|i| self.transform(start + i))
                            .min()
                            .expect("should never panic here");
                    }

                    let step = len / 2;
                    let mid = start + step;
                    let mid_location = self.transform(mid);

                    if self.transform(start) + step != mid_location {
                        queue.push((start, step));
                    }

                    if self.transform(start + len) != mid_location + len - step {
                        queue.push((mid, len - step));
                    }
                }

                i64::MAX
            })
            .min()
            .ok_or(anyhow!("failed to find seed"))
    }
}

aoc::solution!();

#[cfg(test)]
mod test {
    use super::{Solution, Solver};

    const INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_part1() {
        let mut solution = Solution::new(INPUT).unwrap();
        let answer = solution.part1().unwrap().to_string();
        assert_eq!(answer, "35");
    }

    #[test]
    fn test_part2() {
        let mut solution = Solution::new(INPUT).unwrap();
        let answer = solution.part2().unwrap().to_string();
        assert_eq!(answer, "46");
    }
}
