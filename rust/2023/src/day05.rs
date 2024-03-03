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

fn parse(input: &str) -> (Vec<i64>, Vec<Map>) {
    let re = Regex::new(r"\d+").unwrap();
    let mut sections = input.split("\n\n").filter(|section| !section.is_empty());

    let seeds: Vec<i64> = re
        .find_iter(sections.next().unwrap())
        .map(parse_number)
        .collect();

    let maps: Vec<Map> = sections
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

    (seeds, maps)
}

#[allow(dead_code)]
fn part1(input: &str) -> i64 {
    let (seeds, maps) = parse(input);

    seeds
        .par_iter()
        .map(|&seed| maps.iter().fold(seed, |acc, m| m.transform(acc)))
        .min()
        .unwrap()
}

#[allow(dead_code)]
fn part2(input: &str) -> i64 {
    let (seeds, maps) = parse(input);

    seeds
        .par_chunks_exact(2)
        .map(|seed_range| {
            (seed_range[0]..=seed_range[0] + seed_range[1])
                .par_bridge()
                .map(|seed| maps.iter().fold(seed, |acc, m| m.transform(acc)))
                .min()
                .unwrap()
        })
        .min()
        .unwrap()
}

#[allow(dead_code)]
pub fn puzzle() {
    let input = crate::input::load_input(5);
    println!("part 1 = {}", part1(&input));
    println!("part 2 = {}", part2(&input));
}

#[cfg(test)]
mod test {
    use super::{part1, part2};

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
        assert_eq!(part1(INPUT), 35);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 46);
    }
}
