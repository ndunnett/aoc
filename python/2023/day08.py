from __future__ import annotations
from lib import filter_map
from math import lcm
from functools import reduce
from itertools import cycle
from enum import Enum, auto
from typing import Callable


class Direction(Enum):
    L = auto()
    R = auto()

    def parse(ch: str) -> Direction:
        match ch:
            case "L": return Direction.L
            case "R": return Direction.R


class Pair(tuple):
    def __new__(cls, *args) -> Pair:
        return tuple.__new__(cls, args)

    @property
    def left(self):
        return self[0]

    @property
    def right(self):
        return self[1]

    def move(self, direction: Direction) -> str:
        match direction:
            case Direction.L: return self.left
            case Direction.R: return self.right


class State:
    dirs: cycle[Direction]
    map: dict[str, Pair]

    def __init__(self, input: str):
        sections = input.split("\n\n")
        self.dirs = cycle(map(Direction.parse, sections[0]))
        self.map = {
            line[0:3]: Pair(line[7:10], line[12:15])
            for line in sections[1].splitlines()
        }

    def distance(self, start: str, is_target: Callable[[str], bool]) -> int:
        loc, i = start, 1

        while not is_target(loc := self.map[loc].move(next(self.dirs))):
            i += 1

        return i


def part1(input: str) -> int:
    return State(input).distance("AAA", lambda l: l == "ZZZ")


def part2(input: str) -> int:
    s = State(input)

    def _f(loc: str) -> int | None:
        return s.distance(loc, lambda l: l.endswith("Z")) if loc[2] == "A" else None

    return reduce(lcm, filter_map(_f, s.map.keys()))


TEST_INPUT1 = """RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"""

TEST_INPUT2 = """LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"""

TEST_INPUT3 = """LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"""

PART1_TESTS = [
    (TEST_INPUT1, 2),
    (TEST_INPUT2, 6),
]

PART2_TESTS = [
    (TEST_INPUT3, 6),
]
