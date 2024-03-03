use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
enum Face {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Card {
    face: Face,
    value: u32,
}

impl Card {
    fn new(face: &Face, value: u32) -> Card {
        Card { face: *face, value }
    }

    fn from_char(c: char, part2: bool) -> Card {
        match c {
            'A' => Card::new(&Face::Ace, 14),
            'K' => Card::new(&Face::King, 13),
            'Q' => Card::new(&Face::Queen, 12),
            'J' => Card::new(&Face::Jack, if part2 { 1 } else { 11 }),
            'T' => Card::new(&Face::Ten, 10),
            '9' => Card::new(&Face::Nine, 9),
            '8' => Card::new(&Face::Eight, 8),
            '7' => Card::new(&Face::Seven, 7),
            '6' => Card::new(&Face::Six, 6),
            '5' => Card::new(&Face::Five, 5),
            '4' => Card::new(&Face::Four, 4),
            '3' => Card::new(&Face::Three, 3),
            '2' => Card::new(&Face::Two, 2),
            _ => panic!("invalid card character"),
        }
    }
}

#[derive(Debug)]
enum Hand {
    FiveKind(Card, Card, Card, Card, Card),
    FourKind(Card, Card, Card, Card, Card),
    FullHouse(Card, Card, Card, Card, Card),
    ThreeKind(Card, Card, Card, Card, Card),
    TwoPair(Card, Card, Card, Card, Card),
    OnePair(Card, Card, Card, Card, Card),
    HighCard(Card, Card, Card, Card, Card),
}

fn get_card_counts(cards: &[Card], count_map: &HashMap<Card, u32>) -> Vec<u32> {
    cards
        .iter()
        .sorted_by(|a, b| {
            count_map
                .get(b)
                .unwrap()
                .cmp(count_map.get(a).unwrap())
                .then_with(|| b.cmp(a))
        })
        .map(|card| *count_map.get(card).unwrap())
        .collect()
}

impl Hand {
    fn new(cards: &[Card], part2: bool) -> Hand {
        if cards.len() != 5 {
            panic!("failed to parse cards: {:?}", cards)
        }

        let mut jokers = 0;
        let count_map: HashMap<Card, u32> = cards.iter().fold(HashMap::new(), |mut map, &card| {
            if part2 && card.face == Face::Jack {
                jokers += 1;
            }

            *map.entry(card).or_insert(0) += 1;
            map
        });

        let c = get_card_counts(cards, &count_map);

        match (c[0], c[1], c[2], c[3], c[4], jokers) {
            (_, _, _, _, _, 4) => Hand::FiveKind(cards[0], cards[1], cards[2], cards[3], cards[4]),
            (_, _, _, 2, _, 3) => Hand::FiveKind(cards[0], cards[1], cards[2], cards[3], cards[4]),
            (_, _, _, 1, _, 3) => Hand::FourKind(cards[0], cards[1], cards[2], cards[3], cards[4]),
            (3, _, _, 2, _, 2) => Hand::FiveKind(cards[0], cards[1], cards[2], cards[3], cards[4]),
            (_, _, 2, _, _, 2) => Hand::FourKind(cards[0], cards[1], cards[2], cards[3], cards[4]),
            (_, _, 1, _, _, 2) => Hand::ThreeKind(cards[0], cards[1], cards[2], cards[3], cards[4]),
            (4, _, _, _, _, 1) => Hand::FiveKind(cards[0], cards[1], cards[2], cards[3], cards[4]),
            (3, _, _, _, _, 1) => Hand::FourKind(cards[0], cards[1], cards[2], cards[3], cards[4]),
            (2, _, 2, _, _, 1) => Hand::FullHouse(cards[0], cards[1], cards[2], cards[3], cards[4]),
            (2, _, _, 1, _, 1) => Hand::ThreeKind(cards[0], cards[1], cards[2], cards[3], cards[4]),
            (1, _, _, _, _, 1) => Hand::OnePair(cards[0], cards[1], cards[2], cards[3], cards[4]),
            (5, _, _, _, _, _) => Hand::FiveKind(cards[0], cards[1], cards[2], cards[3], cards[4]),
            (4, _, _, _, _, _) => Hand::FourKind(cards[0], cards[1], cards[2], cards[3], cards[4]),
            (3, _, _, 2, _, _) => Hand::FullHouse(cards[0], cards[1], cards[2], cards[3], cards[4]),
            (3, _, _, 1, _, _) => Hand::ThreeKind(cards[0], cards[1], cards[2], cards[3], cards[4]),
            (2, _, 2, _, _, _) => Hand::TwoPair(cards[0], cards[1], cards[2], cards[3], cards[4]),
            (2, _, 1, _, _, _) => Hand::OnePair(cards[0], cards[1], cards[2], cards[3], cards[4]),
            (1, _, _, _, _, _) => Hand::HighCard(cards[0], cards[1], cards[2], cards[3], cards[4]),
            _ => panic!("failed to parse cards: {:?}", cards),
        }
    }

    fn value(&self) -> u32 {
        let f = |x: u32, a: &Card, b: &Card, c: &Card, d: &Card, e: &Card| {
            x * 10_000_000
                + a.value * 15 * 15 * 15 * 15
                + b.value * 15 * 15 * 15
                + c.value * 15 * 15
                + d.value * 15
                + e.value
        };

        match self {
            Hand::FiveKind(a, b, c, d, e) => f(6, a, b, c, d, e),
            Hand::FourKind(a, b, c, d, e) => f(5, a, b, c, d, e),
            Hand::FullHouse(a, b, c, d, e) => f(4, a, b, c, d, e),
            Hand::ThreeKind(a, b, c, d, e) => f(3, a, b, c, d, e),
            Hand::TwoPair(a, b, c, d, e) => f(2, a, b, c, d, e),
            Hand::OnePair(a, b, c, d, e) => f(1, a, b, c, d, e),
            Hand::HighCard(a, b, c, d, e) => f(0, a, b, c, d, e),
        }
    }
}

fn solve(input: &str, part2: bool) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut it = line.split_whitespace();
            let cards: Vec<Card> = it
                .next()
                .unwrap()
                .chars()
                .map(|c| Card::from_char(c, part2))
                .collect();

            let hand = Hand::new(&cards, part2);
            let wager: u32 = it.next().unwrap().parse().expect("invalid wager");

            (hand, wager)
        })
        .sorted_by(|(a, _), (b, _)| a.value().cmp(&b.value()))
        .enumerate()
        .fold(0, |acc, (i, (_, wager))| acc + wager * (i as u32 + 1))
}

#[allow(dead_code)]
pub fn puzzle() {
    let input = crate::input::load_input(7);
    println!("part 1 = {}", solve(&input, false));
    println!("part 2 = {}", solve(&input, true));
}

#[cfg(test)]
mod test {
    use super::solve;

    const INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn test_part1() {
        assert_eq!(solve(INPUT, false), 6440);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve(INPUT, true), 5905);
    }
}
