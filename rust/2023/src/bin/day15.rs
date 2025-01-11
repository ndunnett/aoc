use std::hash::{Hash, Hasher};

fn hash_algorithm<S: AsRef<str>>(s: S) -> u64 {
    s.as_ref()
        .chars()
        .filter(|&c| c != '\n')
        .fold(0, |acc, c| (acc + c as u64) * 17 % 256)
}

#[derive(Clone, Eq, PartialEq)]
struct Lens {
    id: u64,
    hash: u64,
    length: Option<u64>,
}

impl From<&String> for Lens {
    fn from(value: &String) -> Self {
        let (label, length) = value.split_once(['=', '-']).expect("failed to split lens");
        let length = length.parse::<u64>().ok();
        let mut s = FxHasher::default();
        label.hash(&mut s);

        Self {
            id: s.finish(),
            hash: hash_algorithm(label),
            length,
        }
    }
}

#[derive(Clone)]
struct Solution {
    elements: Vec<String>,
}

impl Solver for Solution {
    fn new(input: &str) -> Anyhow<Self> {
        Ok(Self {
            elements: input.par_split(',').map(String::from).collect(),
        })
    }

    fn part1(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self.elements.par_iter().map(hash_algorithm).sum::<u64>())
    }

    fn part2(&mut self) -> Anyhow<impl fmt::Display> {
        let mut boxes: FxHashMap<u64, Vec<Lens>> = FxHashMap::default();

        for lens in self.elements.iter().map(Lens::from) {
            if lens.length.is_some() {
                let bucket = boxes.entry(lens.hash).or_default();

                if let Some((i, _)) = bucket.iter().find_position(|l| l.id == lens.id) {
                    bucket[i] = lens;
                } else {
                    bucket.push(lens);
                }
            } else if let Some(bucket) = boxes.get_mut(&lens.hash) {
                bucket.retain(|l| l.id != lens.id);
            }
        }

        Ok(boxes.par_iter().flat_map_iter(|(h, b)| {
            b.iter()
                .enumerate()
                .map(move |(i, l)| (*h + 1) * (i as u64 + 1) * l.length.unwrap())
        }))
        .map(|test| test.sum::<u64>())
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
