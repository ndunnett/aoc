from lib import Point
from itertools import pairwise


DIGITS = "0123456789"


class State:
    def __init__(self, input: str):
        self.symbols = {}
        self.numbers = {}

        for y, line in enumerate(input.splitlines()):
            digs = []

            for x, ch in enumerate(line):
                if ch in DIGITS:
                    digs.append((Point(x, y), ch))
                elif ch != ".":
                    self.symbols[Point(x, y)] = ch

            stack = [digs[0]]

            for a, b in pairwise(digs + [None]):
                if not b or b[0].x - a[0].x != 1:
                    num = int("".join(map(lambda tup: tup[1], stack)))
                    y = stack[0][0].y

                    self.numbers[stack[0][0], num] = set([
                        Point(_x, _y)
                        for _x in map(lambda tup: tup[0].x, stack)
                        for _y in [y - 1, y + 1]
                    ] + [
                        Point(_x, _y)
                        for _x in [stack[0][0].x - 1, stack[-1][0].x + 1]
                        for _y in [y - 1, y, y + 1]
                    ])

                    stack = [b]
                else:
                    stack.append(b)


def part1(input: str) -> int:
    s = State(input)

    return sum([
        num
        for p in s.symbols
        for (_, num), edge in s.numbers.items()
        if p in edge
    ])


def part2(input: str) -> int:
    s = State(input)
    answer = 0

    for p, sym in s.symbols.items():
        if sym == "*":
            nums = [num for (_, num), edge in s.numbers.items() if p in edge]

            if len(nums) == 2:
                answer += nums[0] * nums[1]

    return answer


TEST_INPUT = """467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."""

PART1_TESTS = [
    (TEST_INPUT, 4361),
]

PART2_TESTS = [
    (TEST_INPUT, 467835),
]
