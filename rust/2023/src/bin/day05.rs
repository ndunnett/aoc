use rayon::prelude::*;
use regex::{Match, Regex};

#[derive(Debug, Clone)]
struct Layer {
    start: i64,
    end: i64,
    offset: i64,
}

#[derive(Debug, Clone)]
struct Map {
    layers: Vec<Layer>,
}

impl Map {
    fn transform(&self, seed: i64) -> i64 {
        self.layers
            .iter()
            .find(|layer| layer.start <= seed && layer.end > seed)
            .map_or(seed, |layer| seed + layer.offset)
    }
}

fn parse_number(m: Match) -> i64 {
    m.as_str().parse().unwrap()
}

pub struct Solution {
    seeds: Vec<i64>,
    maps: Vec<Map>,
}

impl Solver for Solution {
    fn new(input: &str) -> Anyhow<Self> {
        let re = Regex::new(r"\d+")?;
        let mut sections = input.split("\n\n").filter(|section| !section.is_empty());

        let seeds = re
            .find_iter(sections.next().unwrap())
            .map(parse_number)
            .collect();

        let maps = sections
            .map(|section| Map {
                layers: section
                    .lines()
                    .skip(1)
                    .map(|line| {
                        let numbers: Vec<i64> = re.find_iter(line).map(parse_number).collect();
                        Layer {
                            start: numbers[1],
                            end: numbers[1] + numbers[2],
                            offset: numbers[0] - numbers[1],
                        }
                    })
                    .collect(),
            })
            .collect();

        Ok(Self { seeds, maps })
    }

    fn part1(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self
            .seeds
            .par_iter()
            .map(|&seed| self.maps.iter().fold(seed, |acc, m| m.transform(acc)))
            .min()
            .unwrap())
    }

    fn part2(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self
            .seeds
            .par_chunks_exact(2)
            .map(|seed_range| {
                (seed_range[0]..=seed_range[0] + seed_range[1])
                    .par_bridge()
                    .map(|seed| self.maps.iter().fold(seed, |acc, m| m.transform(acc)))
                    .min()
                    .unwrap()
            })
            .min()
            .unwrap())
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
