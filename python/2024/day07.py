from math import log10

from lib import filter_map


def parse_equation(line: str) -> tuple[int, list[int]]:
    (value, operands) = line.split(": ", 1)
    value = int(value)
    operands = [int(n) for n in operands.split(" ")]
    return (value, operands)


def concat(a: int, b: int) -> int:
    return a * 10 ** int(log10(b) + 1) + b


def is_correct(value: int, n: list[int], *, c: bool) -> bool:
    if len(n) == 2:
        return value == n[0] * n[1] or value == n[0] + n[1] or (c and value == concat(n[0], n[1]))
    else:
        return (
            is_correct(value, [n[0] * n[1]] + n[2:], c=c)
            or is_correct(value, [n[0] + n[1]] + n[2:], c=c)
            or (c and is_correct(value, [concat(n[0], n[1])] + n[2:], c=c))
        )


class Equations:
    def __init__(self, input: str) -> None:
        self.equations = [parse_equation(line) for line in input.splitlines()]

    def solve(self, *, concat: bool = False) -> int:
        return sum(
            filter_map(
                lambda eq: eq[0] if is_correct(eq[0], eq[1], c=concat) else None,
                self.equations,
            ),
        )


def part1(input: str) -> int:
    return Equations(input).solve()


def part2(input: str) -> int:
    return Equations(input).solve(concat=True)


TEST_INPUT = """190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"""

PART1_TESTS = [
    (TEST_INPUT, 3749),
]

PART2_TESTS = [
    (TEST_INPUT, 11387),
]
