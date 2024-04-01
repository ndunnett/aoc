from __future__ import annotations
from lib import Point
from functools import cache
from enum import Enum, auto


class Direction(Enum):
    U = auto()
    D = auto()
    L = auto()
    R = auto()

    @cache
    def move(self) -> Point:
        match self:
            case Direction.U: return Point(0, -1)
            case Direction.D: return Point(0, 1)
            case Direction.L: return Point(-1, 0)
            case Direction.R: return Point(1, 0)

    @cache
    def moves(p: Point) -> list[Point]:
        return [p + d.move() for d in Direction._member_map_.values()]


class State(tuple):
    def __new__(cls, *args) -> State:
        return tuple.__new__(cls, args)

    @property
    def point(self) -> Point:
        return self[0]

    @property
    def laps(self) -> Point:
        return self[1]

    @property
    def steps(self) -> int:
        return self[2]

    @property
    def key(self) -> tuple[Point, Point]:
        return self[0:2]


class Map:
    start: Point
    rocks: set[Point]
    size: int
    wraps: bool

    def __init__(self, input: str, wraps: bool = False):
        lines = input.splitlines()
        self.rocks = set()
        self.start = None

        for y, line in enumerate(lines):
            for x, ch in enumerate(line):
                if ch == "#":
                    self.rocks.add(Point(x, y))
                elif not self.start and ch == "S":
                    self.start = Point(x, y)

        self.size = len(lines)
        self.wraps = wraps

    def solve(self, steps: int):
        queue = [State(self.start, Point(0, 0), steps)]
        seen, plots = set({queue[0].key}), 0

        while queue and (state := queue.pop(0)):
            if not state.steps % 2:
                plots += 1

            if not state.steps:
                continue

            for p in Direction.moves(state.point):
                if not (0 <= p.x < self.size and 0 <= p.y < self.size):
                    if self.wraps:
                        np = Point(p.x % self.size, p.y % self.size)
                        nl = state.laps + Point(p.x // self.size, p.y // self.size)
                    else:
                        continue
                else:
                    np, nl = p, state.laps

                if np not in self.rocks and (np, nl) not in seen:
                    queue.append(ns := State(np, nl, state.steps - 1))
                    seen.add(ns.key)

        return plots


def part1(input: str) -> int:
    return Map(input).solve(64)


def part2(input: str) -> int:
    m = Map(input, True)
    steps = 26501365
    n, r = steps // m.size, steps % m.size
    a, b, c = m.solve(r), m.solve(r + m.size), m.solve(r + 2 * m.size)
    return (n ** 2 - n) * ((c + a) // 2 - b) + n * (b - a) + a


TEST_INPUT = """...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
..........."""

PART1_TESTS = [
    # (TEST_INPUT, 16),  # 6 steps
    (TEST_INPUT, 42),  # 64 steps
]
