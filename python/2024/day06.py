from __future__ import annotations

from enum import Enum
from itertools import pairwise

from lib import Point


class Direction(Enum):
    N = 0
    E = 1
    S = 2
    W = 3

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

    def move_point(self, p: Point, size: int) -> Point | None:
        match self:
            case Direction.N if p.y > 0:
                return Point(p.x, p.y - 1)
            case Direction.E if p.x < size - 1:
                return Point(p.x + 1, p.y)
            case Direction.S if p.y < size - 1:
                return Point(p.x, p.y + 1)
            case Direction.W if p.x > 0:
                return Point(p.x - 1, p.y)
            case _:
                return None


class Map:
    obstacles: set[Point]
    size: int
    start: Point

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

    def __iter__(self) -> MapIter:
        return MapIter(self)


class MapIter:
    map_: Map
    direction: Direction
    position: Point | None

    def __init__(self, map_: Map) -> None:
        self.map_ = map_
        self.direction = Direction.N
        self.position = map_.start

    def __iter__(self) -> MapIter:
        return self

    def __next__(self) -> tuple[Point, Direction]:
        if p := self.position:
            if next_ := self.direction.move_point(p, self.map_.size):
                if next_ in self.map_.obstacles:
                    self.direction = self.direction.turn()
                else:
                    self.position = next_
            else:
                self.position = None

            return (p, self.direction)
        else:
            raise StopIteration


def part1(input: str) -> int:
    return len({p for p, _ in Map(input)})


def part2(input: str) -> int:
    m = Map(input)
    count = 0

    for point in {p for p, _ in m}:
        m.obstacles.add(point)
        seen = (set(), set(), set(), set())

        for (ap, ad), (_, bd) in pairwise(m):
            if ad != bd:
                if ap in seen[ad.value]:
                    count += 1
                    break

                seen[ad.value].add(ap)

        m.obstacles.remove(point)

    return count


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
