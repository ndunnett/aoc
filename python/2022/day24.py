from __future__ import annotations
from copy import deepcopy
from enum import Enum, auto
from functools import cache
from collections import defaultdict


class Point(tuple):
    @cache
    def __new__(self, x, y):
        return tuple.__new__(Point, (x, y))

    @property
    def x(self) -> int:
        return self[0]

    @property
    def y(self) -> int:
        return self[1]

    @cache
    def __add__(self, other: Point) -> Point:
        return Point(self.x + other.x, self.y + other.y)

    @cache
    def adjacent(self) -> list[Point]:
        return [
            Point(self.x + 1, self.y),
            Point(self.x - 1, self.y),
            Point(self.x, self.y + 1),
            Point(self.x, self.y - 1),
        ]


class Symbol(Enum):
    N = auto()
    S = auto()
    W = auto()
    E = auto()


SYMBOLS = {
    "^": Symbol.N,
    "v": Symbol.S,
    "<": Symbol.W,
    ">": Symbol.E,
}

MOVES = {
    Symbol.E: Point(1, 0),
    Symbol.W: Point(-1, 0),
    Symbol.S: Point(0, 1),
    Symbol.N: Point(0, -1),
}


class State(defaultdict):
    def __init__(self, *args, **kwargs):
        super(State, self).__init__(list, *args, **kwargs)

    def items_copy(self) -> list[State]:
        return [(k, deepcopy(v)) for k, v in self.items() if len(v) > 0]

    def parse(input: str) -> State:
        lines = input.splitlines()
        height = len(lines) - 2
        width = len(lines[0]) - 2

        s = State({
            Point(x - 1, y - 1): [SYMBOLS[ch]]
            for y, line in enumerate(lines)
            for x, ch in enumerate(line)
            if ch in SYMBOLS and 0 < x <= width and 0 < y <= height
        })

        s.height = height
        s.width = width
        s.start = Point(0, -1)
        s.goal = Point(s.width - 1, s.height)
        return s

    def get_neighbours(self, p: Point) -> list[Point]:
        neighbours = [n for n in p.adjacent() if self.is_valid_move(n)]

        if self.is_valid_move(p):
            neighbours.append(p)

        return neighbours

    def is_valid_move(self, point: Point) -> bool:
        return point in [self.start, self.goal] or (point in self and not self[point])

    def move_to_goal(self) -> int:
        positions = set([self.start])

        for time in range(1, 1000):
            next_positions = set()

            for point, elements in self.items_copy():
                for element in elements:
                    self[point].remove(element)
                    x = (point.x + MOVES[element].x) % self.width
                    y = (point.y + MOVES[element].y) % self.height
                    self[Point(x, y)].append(element)

            for _point in positions:
                for point in self.get_neighbours(_point):
                    if point == self.goal:
                        return time

                    next_positions.add(point)

            positions = next_positions

            if not positions:
                positions.add(self.start)


def part1(input: str) -> int:
    return State.parse(input).move_to_goal()


def part2(input: str) -> int:
    s = State.parse(input)
    answer = s.move_to_goal()

    for _ in range(2):
        s.start, s.goal = s.goal, s.start
        answer += s.move_to_goal()

    return answer


TEST_INPUT = """#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#"""

PART1_TESTS = [
    (TEST_INPUT, 18),
]

PART2_TESTS = [
    (TEST_INPUT, 54),
]
