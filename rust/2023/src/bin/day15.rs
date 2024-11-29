use std::{
    collections::hash_map::Entry,
    hash::{DefaultHasher, Hasher},
};

use itertools::Itertools;

fn hash<S: AsRef<str>>(s: S) -> usize {
    s.as_ref()
        .chars()
        .filter(|&c| c != '\n')
        .fold(0, |acc, c| (acc + c as usize) * 17 % 256)
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Lens {
    id: u64,
    hash: usize,
    length: Option<usize>,
}

impl Lens {
    fn new<S: AsRef<str> + std::hash::Hash>(label: S, length: Option<usize>) -> Self {
        let mut s = DefaultHasher::new();
        label.hash(&mut s);

        Self {
            id: s.finish(),
            hash: hash(label),
            length,
        }
    }

    fn parse<S: AsRef<str>>(chunk: S) -> Lens {
        let (label, length) = chunk.as_ref().split_once(['=', '-']).unwrap();
        let length = length.parse::<usize>().ok();
        Self::new(label, length)
    }
}

pub struct Solution {
    elements: Vec<String>,
}

impl Solver for Solution {
    fn new(input: &str) -> Anyhow<Self> {
        Ok(Self {
            elements: input.split(',').map(String::from).collect(),
        })
    }

    fn part1(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self.elements.iter().map(hash).sum::<usize>())
    }

    fn part2(&mut self) -> Anyhow<impl fmt::Display> {
        let mut boxes: HashMap<usize, Vec<Lens>> = HashMap::new();

        for lens in self.elements.iter().map(Lens::parse) {
            if lens.length.is_some() {
                let bucket = boxes.entry(lens.hash).or_default();

                if let Some((i, _)) = bucket.iter().find_position(|l| l.id == lens.id) {
                    bucket[i] = lens;
                } else {
                    bucket.push(lens);
                }
            } else if let Entry::Occupied(mut entry) = boxes.entry(lens.hash) {
                entry.get_mut().retain(|l| l.id != lens.id);
            }
        }

        Ok(boxes
            .iter()
            .flat_map(|(h, b)| {
                b.iter()
                    .enumerate()
                    .map(move |(i, l)| (h + 1) * (i + 1) * l.length.unwrap())
            })
            .sum::<usize>())
    }
}

aoc::solution!();

#[cfg(test)]
mod test {
    use super::{Solution, Solver};

    const INPUT1: &str = "HASH";
    const INPUT2: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_part1_1() {
        let mut solution = Solution::new(INPUT1).unwrap();
        let answer = solution.part1().unwrap().to_string();
        assert_eq!(answer, "52");
    }

    #[test]
    fn test_part1_2() {
        let mut solution = Solution::new(INPUT2).unwrap();
        let answer = solution.part1().unwrap().to_string();
        assert_eq!(answer, "1320");
    }

    #[test]
    fn test_part2() {
        let mut solution = Solution::new(INPUT2).unwrap();
        let answer = solution.part2().unwrap().to_string();
        assert_eq!(answer, "145");
    }
}
