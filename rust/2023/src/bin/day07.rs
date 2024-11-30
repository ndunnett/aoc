#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl TryFrom<char> for Card {
    type Error = Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'A' => Ok(Self::Ace),
            'K' => Ok(Self::King),
            'Q' => Ok(Self::Queen),
            'J' => Ok(Self::Jack),
            'T' => Ok(Self::Ten),
            '9' => Ok(Self::Nine),
            '8' => Ok(Self::Eight),
            '7' => Ok(Self::Seven),
            '6' => Ok(Self::Six),
            '5' => Ok(Self::Five),
            '4' => Ok(Self::Four),
            '3' => Ok(Self::Three),
            '2' => Ok(Self::Two),
            '?' => Ok(Self::Joker),
            _ => Err(anyhow!("invalid card character: '{c}'")),
        }
    }
}

enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

struct Hand {
    cards: [Card; 5],
    wager: u32,
}

impl TryFrom<&String> for Hand {
    type Error = Error;

    fn try_from(line: &String) -> Result<Self, Self::Error> {
        let (cards, wager) = line
            .split_once(' ')
            .ok_or(anyhow!("failed to parse line"))?;

        let cards = cards
            .chars()
            .map(Card::try_from)
            .collect::<Anyhow<Vec<_>>>()?
            .try_into()
            .map_err(|_| anyhow!("failed to parse cards into array"))?;

        let wager = wager.parse::<u32>()?;

        Ok(Self { cards, wager })
    }
}

impl TryFrom<String> for Hand {
    type Error = Error;

    fn try_from(line: String) -> Result<Self, Self::Error> {
        Self::try_from(&line)
    }
}

impl Hand {
    fn value(&self) -> u64 {
        let mut counts = [0_u8; 14];

        for &card in self.cards.iter() {
            counts[card as usize] += 1;
        }

        let jokers = counts[Card::Joker as usize];

        counts.sort();
        counts.reverse();

        let hand_type = match (&counts[..5], jokers) {
            ([4, 1, _, _, _], 4) => HandType::FiveKind,
            ([3, 2, _, _, _], 3) => HandType::FiveKind,
            ([3, 1, 1, _, _], 3) => HandType::FourKind,
            ([3, 2, _, _, _], 2) => HandType::FiveKind,
            ([2, 2, 1, _, _], 2) => HandType::FourKind,
            ([2, 1, 1, 1, _], 2) => HandType::ThreeKind,
            ([4, 1, _, _, _], 1) => HandType::FiveKind,
            ([3, 1, 1, _, _], 1) => HandType::FourKind,
            ([2, 2, 1, _, _], 1) => HandType::FullHouse,
            ([2, 1, 1, 1, _], 1) => HandType::ThreeKind,
            ([1, 1, 1, 1, 1], 1) => HandType::OnePair,
            ([5, _, _, _, _], _) => HandType::FiveKind,
            ([4, 1, _, _, _], _) => HandType::FourKind,
            ([3, 2, _, _, _], _) => HandType::FullHouse,
            ([3, 1, 1, _, _], _) => HandType::ThreeKind,
            ([2, 2, 1, _, _], _) => HandType::TwoPair,
            ([2, 1, 1, 1, _], _) => HandType::OnePair,
            ([1, 1, 1, 1, 1], _) => HandType::HighCard,
            _ => unreachable!(),
        };

        let mut value = (hand_type as u64) << (8 * 5);

        for (i, &card) in self.cards.iter().rev().enumerate() {
            value |= (card as u64) << (8 * i);
        }

        value
    }
}

struct Solution {
    lines: Vec<String>,
}

impl Solver for Solution {
    fn new(input: &str) -> Anyhow<Self> {
        Ok(Self {
            lines: input.lines().map(String::from).collect(),
        })
    }

    fn part1(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self
            .lines
            .iter()
            .map(Hand::try_from)
            .collect::<Anyhow<Vec<_>>>()?
            .iter()
            .sorted_by_cached_key(|card| card.value())
            .enumerate()
            .fold(0, |acc, (i, hand)| acc + hand.wager * (i as u32 + 1)))
    }

    fn part2(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self
            .lines
            .iter()
            .map(|s| s.replace('J', "?"))
            .map(Hand::try_from)
            .collect::<Anyhow<Vec<_>>>()?
            .iter()
            .sorted_by_cached_key(|card| card.value())
            .enumerate()
            .fold(0, |acc, (i, hand)| acc + hand.wager * (i as u32 + 1)))
    }
}

aoc::solution!();

#[cfg(test)]
mod test {
    use super::{Solution, Solver};

    const INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn test_part1() {
        let mut solution = Solution::new(INPUT).unwrap();
        let answer = solution.part1().unwrap().to_string();
        assert_eq!(answer, "6440");
    }

    #[test]
    fn test_part2() {
        let mut solution = Solution::new(INPUT).unwrap();
        let answer = solution.part2().unwrap().to_string();
        assert_eq!(answer, "5905");
    }
}
