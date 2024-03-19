from itertools import pairwise


def parse(input: str) -> list[list[int]]:
    return [[int(s) for s in line.split()] for line in input.splitlines()]


def decompose(comp: list[list[int]]) -> list[list[int]]:
    while len(set(comp[-1])) > 1:
        comp.append([b - a for a, b in pairwise(comp[-1])])
    return comp


def predict(hist: list[int]) -> int:
    return sum(map(lambda d: d[-1], decompose([hist])))


def part1(input: str) -> int:
    return sum(map(predict, parse(input)))


def part2(input: str) -> int:
    return sum(map(lambda l: predict(list(reversed(l))), parse(input)))


TEST_INPUT = """0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"""

PART1_TESTS = [
    (TEST_INPUT, 114),
]

PART2_TESTS = [
    (TEST_INPUT, 2),
]
