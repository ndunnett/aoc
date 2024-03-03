use itertools::Itertools;
use std::collections::{hash_map::Entry, HashMap};

#[derive(Debug, Clone, Eq, PartialEq)]
struct Lens<'a> {
    label: &'a str,
    length: Option<usize>,
}

impl Lens<'_> {
    fn new(label: &str, length: Option<usize>) -> Lens {
        Lens { label, length }
    }

    fn parse(chunk: &str) -> Lens {
        let (label, length) = chunk.split_once(|c| c == '=' || c == '-').unwrap();
        let length = length.parse::<usize>().ok();
        Lens::new(label, length)
    }
}

fn hash(s: &str) -> usize {
    s.chars()
        .filter(|&c| c != '\n')
        .fold(0, |acc, c| (acc + c as usize) * 17 % 256)
}

fn part1(input: &str) -> usize {
    input.split(',').map(hash).sum()
}

fn part2(input: &str) -> usize {
    let mut boxes: HashMap<usize, Vec<Lens>> = HashMap::new();

    for lens in input.split(',').map(Lens::parse) {
        let hash = hash(lens.label);

        if lens.length.is_some() {
            let bucket = boxes.entry(hash).or_default();

            if let Some((i, _)) = bucket.iter().find_position(|l| l.label == lens.label) {
                bucket[i] = lens;
            } else {
                bucket.push(lens);
            }
        } else if let Entry::Occupied(mut entry) = boxes.entry(hash) {
            entry.get_mut().retain(|l| l.label != lens.label);
        }
    }

    boxes
        .iter()
        .flat_map(|(h, b)| {
            b.iter()
                .enumerate()
                .map(move |(i, l)| (h + 1) * (i + 1) * l.length.unwrap())
        })
        .sum()
}

#[allow(dead_code)]
pub fn puzzle() {
    let input = crate::input::load_input(15);
    println!("part 1 = {:?}", part1(&input));
    println!("part 2 = {:?}", part2(&input));
}

#[cfg(test)]
mod test {
    use super::{part1, part2};

    const INPUT1: &str = "HASH";
    const INPUT2: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT1), 52);
        assert_eq!(part1(INPUT2), 1320);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT2), 145);
    }
}
