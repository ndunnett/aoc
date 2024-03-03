from __future__ import annotations
from functools import cache
from enum import Enum, auto
from itertools import chain, pairwise
import re


RE = re.compile(r"\d+|[RL]")


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
    def adjacent(self, heading: Heading) -> Point:
        match heading:
            case Heading.R: return Point(self.x + 1, self.y)
            case Heading.L: return Point(self.x - 1, self.y)
            case Heading.D: return Point(self.x, self.y + 1)
            case Heading.U: return Point(self.x, self.y - 1)


class Rotate(Enum):
    L = auto()
    R = auto()


class Tile(Enum):
    Open = auto()
    Closed = auto()

    def parse(ch: str) -> Tile | None:
        match ch:
            case ".": return Tile.Open
            case "#": return Tile.Closed
            case _: return None


class Heading(Enum):
    R = 0
    D = 1
    L = 2
    U = 3

    def turn_left(self) -> Heading:
        match self:
            case Heading.R: return Heading.U
            case Heading.D: return Heading.R
            case Heading.L: return Heading.D
            case Heading.U: return Heading.L

    def turn_right(self) -> Heading:
        match self:
            case Heading.R: return Heading.D
            case Heading.D: return Heading.L
            case Heading.L: return Heading.U
            case Heading.U: return Heading.R

    def turn_back(self) -> Heading:
        match self:
            case Heading.R: return Heading.L
            case Heading.D: return Heading.U
            case Heading.L: return Heading.R
            case Heading.U: return Heading.D

    def turn(self, rotation: Rotate) -> Heading:
        match rotation:
            case Rotate.L: return self.turn_left()
            case Rotate.R: return self.turn_right()


def is_number(s: str) -> bool:
    try:
        int(s)
        return True
    except:
        return False


class State:
    path: list[Rotate | int]
    board: dict[Point, Tile]
    heading: Heading
    pos: Point
    transitions: dict[tuple[Point, Heading], tuple[Point, Heading]]

    def __init__(self, input: str):
        [board, path] = input.split("\n\n")

        self.board = {
            Point(x, y): tile
            for y, line in enumerate(board.splitlines())
            for x, element in enumerate(line)
            if (tile := Tile.parse(element))
        }

        self.path = [int(x) if is_number(x) else Rotate._member_map_[x] for x in RE.findall(path)]
        self.heading = Heading.R
        self.pos = Point(min(p.x for p in self.board if p.y == 0), 0)
        self.transitions = self.parse_transitions()

    def parse_transitions(self) -> dict[tuple[Point, Heading], tuple[Point, Heading]]:
        raise NotImplementedError()

    def trace_edge(self) -> list[tuple[Point, Heading]]:
        trace = []
        heading = self.heading
        pos = self.pos

        while True:
            trace.append((pos, heading.turn_left()))
            next_pos = pos.adjacent(heading)

            if next_pos in self.board:
                left_heading = heading.turn_left()
                left_pos = next_pos.adjacent(left_heading)

                if left_pos in self.board:
                    pos = left_pos
                    heading = left_heading
                else:
                    pos = next_pos

            else:
                heading = heading.turn_right()

            if pos == self.pos and heading == self.heading:
                return trace

    def solve(self) -> int:
        for movement in self.path:
            if isinstance(movement, int):
                for _ in range(movement):
                    if (next_pos := self.pos.adjacent(self.heading)) not in self.board:
                        next_pos, next_heading = self.transitions[self.pos, self.heading]
                    else:
                        next_heading = self.heading

                    if self.board[next_pos] == Tile.Open:
                        self.pos = next_pos
                        self.heading = next_heading
                    else:
                        break

            else:
                self.heading = self.heading.turn(movement)

        return 1000 * (self.pos.y + 1) + 4 * (self.pos.x + 1) + self.heading.value


class FlatState(State):
    def parse_transitions(self) -> dict[tuple[Point, Heading], tuple[Point, Heading]]:
        edges = self.trace_edge()

        def y(tup): return tup[0].y
        def x(tup): return tup[0].x
        def fs(heading, key): return sorted(filter(lambda e: e[1] == heading, edges), key=key)

        pairs = chain(zip(fs(Heading.R, y), fs(Heading.L, y)), zip(fs(Heading.D, x), fs(Heading.U, x)))
        transitions = {}

        for (pos_a, heading_a), (pos_b, heading_b) in pairs:
            transitions[pos_a, heading_a] = pos_b, heading_b.turn_back()
            transitions[pos_b, heading_b] = pos_a, heading_a.turn_back()

        return transitions


class CubeState(State):
    def parse_transitions(self) -> dict[tuple[Point, Heading], tuple[Point, Heading]]:
        edges = self.trace_edge()
        side_length = len(edges) // 14
        sides = [(edges[i][1].turn_right(), edges[i:i + side_length]) for i in range(0, len(edges), side_length)]
        pairs = []

        while (queue := list(pairwise(sides))):
            for i, ((heading_a, side_a), (heading_b, side_b)) in enumerate(queue):
                if heading_a == heading_b.turn_right():
                    pairs.append((side_a, side_b))
                    sides[i:] = [(heading.turn_left(), side) for heading, side in sides[i + 2:]]
                    break

        transitions = {}

        for side_a, side_b in pairs:
            for (pos_a, heading_a), (pos_b, heading_b) in zip(side_a, reversed(side_b)):
                transitions[pos_a, heading_a] = pos_b, heading_b.turn_back()
                transitions[pos_b, heading_b] = pos_a, heading_a.turn_back()

        return transitions


def part1(input: str) -> int:
    return FlatState(input).solve()


def part2(input: str) -> int:
    return CubeState(input).solve()


TEST_INPUT = """        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5
"""

PART1_TESTS = [
    (TEST_INPUT, 6032),
]

PART2_TESTS = [
    (TEST_INPUT, 5031),
]
