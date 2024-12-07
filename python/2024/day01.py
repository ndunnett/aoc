from functools import reduce


class State:
    def __init__(self, input: str) -> None:
        self.a, self.b = [], []

        for line in input.splitlines():
            (a, b) = line.split("   ")
            self.a.append(int(a))
            self.b.append(int(b))

        self.a.sort()
        self.b.sort()

    def total_distance(self) -> int:
        return reduce(lambda acc, pair: acc + abs(pair[0] - pair[1]), zip(self.a, self.b), 0)

    def similarity(self) -> int:
        b_counts = {b: self.b.count(b) for b in set(self.b)}
        return sum((b_counts.get(a) or 0) * a for a in self.a)


def part1(input: str) -> int:
    return State(input).total_distance()


def part2(input: str) -> int:
    return State(input).similarity()


TEST_INPUT = """3   4
4   3
2   5
1   3
3   9
3   3"""

PART1_TESTS = [
    (TEST_INPUT, 11),
]

PART2_TESTS = [
    (TEST_INPUT, 31),
]
