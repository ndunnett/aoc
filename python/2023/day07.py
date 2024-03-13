from __future__ import annotations
from enum import Enum, auto
from functools import cache


class Card(Enum):
    Joker = auto()
    Two = auto()
    Three = auto()
    Four = auto()
    Five = auto()
    Six = auto()
    Seven = auto()
    Eight = auto()
    Nine = auto()
    Ten = auto()
    Jack = auto()
    Queen = auto()
    King = auto()
    Ace = auto()

    @cache
    def from_char(ch: str) -> Card:
        match ch:
            case "?": return Card.Joker
            case "2": return Card.Two
            case "3": return Card.Three
            case "4": return Card.Four
            case "5": return Card.Five
            case "6": return Card.Six
            case "7": return Card.Seven
            case "8": return Card.Eight
            case "9": return Card.Nine
            case "T": return Card.Ten
            case "J": return Card.Jack
            case "Q": return Card.Queen
            case "K": return Card.King
            case "A": return Card.Ace

    def __lt__(l, r: Card) -> bool:
        return l.value < r.value


class HandType(Enum):
    HighCard = auto()
    OnePair = auto()
    TwoPair = auto()
    ThreeKind = auto()
    FullHouse = auto()
    FourKind = auto()
    FiveKind = auto()

    @cache
    def from_counts(counts: tuple[int, int, int, int, int], jokers: int) -> HandType:
        match (counts, jokers):
            case ((_, _, _, _, _), 4): return HandType.FiveKind
            case ((_, 2, _, _, _), 3): return HandType.FiveKind
            case ((_, 1, _, _, _), 3): return HandType.FourKind
            case ((_, 2, _, _, 3), 2): return HandType.FiveKind
            case ((_, _, 2, _, _), 2): return HandType.FourKind
            case ((_, _, 1, _, _), 2): return HandType.ThreeKind
            case ((_, _, _, _, 4), 1): return HandType.FiveKind
            case ((_, _, _, _, 3), 1): return HandType.FourKind
            case ((_, _, 2, _, 2), 1): return HandType.FullHouse
            case ((_, _, 1, _, 2), 1): return HandType.ThreeKind
            case ((_, _, _, _, 1), 1): return HandType.OnePair
            case ((_, _, _, _, 5), _): return HandType.FiveKind
            case ((_, _, _, _, 4), _): return HandType.FourKind
            case ((_, 2, _, _, 3), _): return HandType.FullHouse
            case ((_, 1, _, _, 3), _): return HandType.ThreeKind
            case ((_, _, 2, _, 2), _): return HandType.TwoPair
            case ((_, _, 1, _, 2), _): return HandType.OnePair
            case ((_, _, _, _, 1), _): return HandType.HighCard


MAX_CARD = Card.Ace.value


class Hand:
    cards: tuple[Card, Card, Card, Card, Card]
    bid: int
    score: int

    def __init__(self, line: str):
        s = line.split()
        self.cards = tuple(map(Card.from_char, s[0]))
        self.bid = int(s[1])
        counts = tuple(sorted(map(self.count_cards, self.cards)))
        hand_type = HandType.from_counts(counts, self.count_cards(Card.Joker))
        self.score = hand_type.value * MAX_CARD ** 5 + \
            sum(c.value * MAX_CARD ** (4 - i) for i, c in enumerate(self.cards))

    def count_cards(self, card: Card) -> int:
        return sum(True for c in self.cards if c == card)


def solve(input: str) -> int:
    def winnings(tup: tuple[int, Hand]) -> int:
        return (1 + tup[0]) * tup[1].bid

    return sum(map(winnings, enumerate(sorted(map(Hand, input.splitlines()), key=lambda h: h.score))))


def part1(input: str) -> int:
    return solve(input)


def part2(input: str) -> int:
    return solve(input.replace("J", "?"))


TEST_INPUT = """32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"""

PART1_TESTS = [
    (TEST_INPUT, 6440),
]

PART2_TESTS = [
    (TEST_INPUT, 5905),
]
