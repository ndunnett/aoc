from __future__ import annotations

from enum import Enum, auto
from typing import TYPE_CHECKING

from lib import Point, chunk_by, peek, peekable

if TYPE_CHECKING:
    from collections.abc import Generator


class Direction(Enum):
    N = auto()
    E = auto()
    S = auto()
    W = auto()

    def turn(self) -> Direction:
        match self:
            case Direction.N:
                return Direction.E
            case Direction.E:
                return Direction.S
            case Direction.S:
                return Direction.W
            case Direction.W:
                return Direction.N


class Map:
    def __init__(self, input: str) -> None:
        self.obstacles = set()

        for y, line in enumerate(input.splitlines()):
            for x, ch in enumerate(line):
                if ch == "#":
                    self.obstacles.add(Point(x, y))
                elif ch == "^":
                    self.start = Point(x, y)

        if self.start is None:
            raise RuntimeError("failed to find start point")

        self.size = len(input.splitlines())

    def get_index(self, s: Point[int], dir: Direction) -> Generator[tuple[Point[int], Direction]]:
        match dir:
            case Direction.N:
                return ((Point(s.x, y), dir) for y in range(s.y, -1, -1))
            case Direction.E:
                return ((Point(x, s.y), dir) for x in range(s.x, self.size))
            case Direction.S:
                return ((Point(s.x, y), dir) for y in range(s.y, self.size))
            case Direction.W:
                return ((Point(x, s.y), dir) for x in range(s.x, -1, -1))

    def __iter__(self) -> MapIter:
        return MapIter(self)


class MapIter:
    def __init__(self, map_: Map) -> None:
        self.map_ = map_
        self.index = peekable(self.map_.get_index(self.map_.start, Direction.N))

    def __iter__(self) -> MapIter:
        return iter(self.map_)

    def __next__(self) -> tuple[Point[int], Direction]:
        if next_ := next(self.index):
            (point, direction) = next_

            if peeked := peek(self.index):
                (peeked, _) = peeked

                if peeked in self.map_.obstacles:
                    turned = direction.turn()
                    self.index = peekable(self.map_.get_index(point, turned))
                    return (point, turned)
                else:
                    return (point, direction)
            else:
                return (point, direction)

        raise StopIteration


def part1(input: str) -> int:
    return len({point for point, _ in Map(input)})


def part2(input: str) -> int:
    map_ = Map(input)
    points = {point for point, _ in map_ if point != map_.start}
    result = -1

    for point in points:
        map_.obstacles.add(point)
        path = set()

        for _, chunk in chunk_by(lambda tup: tup[1], map_):
            if chunk[-1] in path:
                result += 1
                break

            path.add(chunk[-1])

        map_.obstacles.remove(point)

    return result


TEST_INPUT = """....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."""

PART1_TESTS = [
    (TEST_INPUT, 41),
]

PART2_TESTS = [
    (TEST_INPUT, 6),
]
