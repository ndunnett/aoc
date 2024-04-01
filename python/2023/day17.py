from __future__ import annotations
from lib import Point
from functools import cache
from enum import Enum, auto
from heapq import heappop, heappush


class Direction(Enum):
    N = auto()
    S = auto()
    E = auto()
    W = auto()

    @cache
    def move(self) -> Point:
        match self:
            case Direction.N: return Point(0, -1)
            case Direction.S: return Point(0, 1)
            case Direction.E: return Point(1, 0)
            case Direction.W: return Point(-1, 0)

    @cache
    def turns(self) -> tuple[Direction]:
        match self:
            case Direction.N | Direction.S: return (Direction.E, Direction.W)
            case Direction.E | Direction.W: return (Direction.N, Direction.S)


class State(tuple):
    @cache
    def __new__(cls, *args) -> State:
        return tuple.__new__(cls, args)

    @property
    def heat(self) -> int:
        return self[0]

    @property
    def pos(self) -> Point:
        return self[1]

    @property
    def dir(self) -> Direction:
        return self[2]

    @property
    def key(self) -> tuple:
        return self[1:]

    def __lt__(self, other: State) -> bool:
        return self.heat < other.heat


def solve(input: str, max_steps: int, min_steps: int = 1) -> int:
    hm = {
        Point(x, y): int(ch)
        for y, line in enumerate(input.splitlines())
        for x, ch in enumerate(line)
    }

    s = len(hm) ** 0.5 - 1
    target = Point(s, s)
    visited = set()

    queue: list[State] = [
        State(0, Point(0, 0), Direction.N, 0),
        State(0, Point(0, 0), Direction.W, 0),
    ]

    while queue and (s := heappop(queue)):
        if s.key in visited:
            continue

        if s.pos == target:
            return s.heat

        visited.add(s.key)

        for next_dir in s.dir.turns():
            if (next_pos := s.pos + next_dir.move() * (min_steps - 1)) not in hm:
                continue

            next_heat = sum(hm[s.pos + next_dir.move() * n] for n in range(1, min_steps))

            for _ in range(min_steps, max_steps + 1):
                if (next_pos := next_pos + next_dir.move()) in hm:
                    heappush(queue, State(
                        s.heat + (next_heat := next_heat + hm[next_pos]),
                        next_pos,
                        next_dir,
                    ))


def part1(input: str) -> int:
    return solve(input, 3)


def part2(input: str) -> int:
    return solve(input, 10, 4)


TEST_INPUT = """2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"""

PART1_TESTS = [
    (TEST_INPUT, 102),
]

PART2_TESTS = [
    (TEST_INPUT, 94),
]
