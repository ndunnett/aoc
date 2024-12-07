from collections import defaultdict
from collections.abc import Iterable
from functools import cmp_to_key, reduce
from itertools import pairwise


class State:
    def __init__(self, input: str) -> None:
        (rules, updates) = input.split("\n\n", 1)

        self.before = defaultdict(set)
        self.after = defaultdict(set)

        for line in rules.splitlines():
            [a1, a2, _, b1, b2] = line
            a = int(a1) * 10 + int(a2)
            b = int(b1) * 10 + int(b2)
            self.before[a].add(b)
            self.after[b].add(a)

        self.updates = [list(map(int, line.split(","))) for line in updates.splitlines()]

    def compare(self, a: int, b: int) -> int:
        if a in self.before[b] or b in self.after[a]:
            return 0
        else:
            return -1

    def is_sorted(self, update: Iterable[int]) -> bool:
        return all(self.compare(a, b) != 0 for a, b in pairwise(update))

    def correct(self) -> int:
        return reduce(
            lambda acc, update: acc + update[len(update) // 2],
            filter(lambda update: self.is_sorted(update), self.updates),
            0,
        )

    def incorrect(self) -> int:
        return reduce(
            lambda acc, update: acc
            + sorted(update, key=cmp_to_key(self.compare))[len(update) // 2],
            filter(lambda update: not self.is_sorted(update), self.updates),
            0,
        )


def part1(input: str) -> int:
    return State(input).correct()


def part2(input: str) -> int:
    return State(input).incorrect()


TEST_INPUT = """47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
"""

PART1_TESTS = [
    (TEST_INPUT, 143),
]

PART2_TESTS = [
    (TEST_INPUT, 123),
]
