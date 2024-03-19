from __future__ import annotations
from lib import Point, get_limits
from itertools import combinations


class Universe(list):
    def parse(input: str) -> Universe:
        return Universe(
            Point(x, y)
            for y, line in enumerate(input.splitlines())
            for x, ch in enumerate(line)
            if ch == "#"
        )

    def expanded(self, age: int) -> Universe:
        min_x, max_x = get_limits(map(lambda k: k.x, self))
        min_y, max_y = get_limits(map(lambda k: k.y, self))
        cols = list(filter(lambda x: all(x != other.x for other in self), range(min_x, max_x + 1)))
        rows = list(filter(lambda y: all(y != other.y for other in self), range(min_y, max_y + 1)))

        def _e(p: Point) -> Point:
            x = (age - 1) * len(list(filter(lambda x: x < p.x, cols)))
            y = (age - 1) * len(list(filter(lambda y: y < p.y, rows)))
            return p + Point(x, y)

        return Universe(map(_e, self))

    def solve(self) -> int:
        return sum(map(lambda c: abs(c[0].x - c[1].x) + abs(c[0].y - c[1].y), combinations(self, 2)))


def part1(input: str) -> int:
    return Universe.parse(input).expanded(2).solve()


def part2(input: str) -> int:
    return Universe.parse(input).expanded(1_000_000).solve()


TEST_INPUT = """...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."""

PART1_TESTS = [
    (TEST_INPUT, 374),
]

PART2_TESTS = [
    # (TEST_INPUT, 1030),  # 10 times larger
    # (TEST_INPUT, 8410),  # 100 times larger
    (TEST_INPUT, 82000210),  # 1_000_000 times larger
]
