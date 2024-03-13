from lib import filter_map
from functools import reduce


class Card:
    winners: set[int]
    picks: set[int]

    def __init__(self, line: str):
        s = line.split(":")[-1].split("|")
        self.winners = set(map(int, s[0].split()))
        self.picks = set(map(int, s[1].split()))

    def matches(self) -> int:
        return len(self.winners & self.picks)


def part1(input: str) -> int:
    return sum(2 ** (m - 1) for line in input.splitlines() if (m := Card(line).matches()) > 0)


def part2(input: str) -> int:
    def f(acc: tuple[int, list[int]], line: str) -> tuple[int, list[int]]:
        [sum, mult] = acc
        n = 1 + len(mult)
        bonus = [m] * n if (m := Card(line).matches()) > 0 else []
        def fm(x): return x - 1 if x > 1 else None
        return (sum + n, list(filter_map(fm, mult)) + bonus)

    return reduce(f, input.splitlines(), (0, []))[0]


TEST_INPUT = """Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"""

PART1_TESTS = [
    (TEST_INPUT, 13),
]

PART2_TESTS = [
    (TEST_INPUT, 30),
]
