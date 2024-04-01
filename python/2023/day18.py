from lib import Point
from functools import cache
from itertools import pairwise
from enum import Enum, auto
from typing import Callable


class Direction(Enum):
    R = auto()
    D = auto()
    L = auto()
    U = auto()

    @cache
    def parse(ch: str) -> Point:
        match ch:
            case "R" | "0": return Direction.R
            case "D" | "1": return Direction.D
            case "L" | "2": return Direction.L
            case "U" | "3": return Direction.U

    @cache
    def move(self) -> Point:
        match self:
            case Direction.R: return Point(1, 0)
            case Direction.D: return Point(0, 1)
            case Direction.L: return Point(-1, 0)
            case Direction.U: return Point(0, -1)


class Plan(list):
    perimeter: int = 0

    def __init__(self, input: str, parser: Callable[[str], tuple[Direction, int]]):
        pos = Point(0, 0)

        for dir, n in map(parser, input.splitlines()):
            self.append(pos := pos + dir.move() * n)
            self.perimeter += n

    def area(self) -> int:
        return (sum(a.x * b.y - b.x * a.y for a, b in pairwise(self)) - self.perimeter) // 2 + self.perimeter + 1


def parse_simple(line: str) -> tuple[Direction, int]:
    s = line.split()
    return (Direction.parse(s[0]), int(s[1]))


def parse_hex(line: str) -> tuple[Direction, int]:
    s = line.split()
    return (Direction.parse(s[2][7]), int(s[2][2:7], 16))


def part1(input: str) -> int:
    return Plan(input, parse_simple).area()


def part2(input: str) -> int:
    return Plan(input, parse_hex).area()


TEST_INPUT = """R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"""

PART1_TESTS = [
    (TEST_INPUT, 62),
]

PART2_TESTS = [
    (TEST_INPUT, 952408144115),
]
