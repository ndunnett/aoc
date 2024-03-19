from __future__ import annotations
from lib import Point
from enum import Enum, auto
from dataclasses import dataclass


class Direction(Enum):
    N = auto()
    S = auto()
    E = auto()
    W = auto()

    def move(self) -> Point:
        match self:
            case Direction.N: return Point(0, -1)
            case Direction.S: return Point(0, 1)
            case Direction.E: return Point(1, 0)
            case Direction.W: return Point(-1, 0)

    def ranges(self, w: int, h: int) -> tuple[range, range]:
        match self:
            case Direction.N: return (range(w), range(h))
            case Direction.S: return (range(w), range(h - 1, -1, -1))
            case Direction.E: return (range(w - 1, -1, -1), range(h))
            case Direction.W: return (range(w), range(h))


@dataclass(frozen=True)
class Platform:
    round_rocks: frozenset[Point]
    square_rocks: frozenset[Point]
    w: int
    h: int

    def parse(input: str) -> Platform:
        round_rocks, square_rocks = set(), set()

        for y, line in enumerate(lines := input.splitlines()):
            for x, ch in enumerate(line):
                if ch == "O":
                    round_rocks.add(Point(x, y))
                elif ch == "#":
                    square_rocks.add(Point(x, y))

        return Platform(frozenset(round_rocks), frozenset(square_rocks), len(lines[0]), len(lines))

    def tilted(self, direction: Direction) -> Platform:
        move_p = direction.move()
        range_x, range_y = direction.ranges(self.w, self.h)
        moved = set()

        for y in range_y:
            for x in range_x:
                p = Point(x, y)

                if p in self.round_rocks:
                    new_p = p

                    while (next_p := new_p + move_p).x in range_x and next_p.y in range_y and next_p not in self.square_rocks and next_p not in moved:
                        new_p = next_p

                    moved.add(new_p)

        return Platform(frozenset(moved), self.square_rocks, self.w, self.h)

    def cycled(self, n: int) -> Platform:
        pf = self
        cache = {}

        for i in range(n):
            if (k := hash(pf.round_rocks)) in cache and (n - i) % (i - cache[k]) == 0:
                return pf

            cache[k] = i

            for direction in [Direction.N, Direction.W, Direction.S, Direction.E]:
                pf = pf.tilted(direction)

        return pf

    def weight(self) -> int:
        return sum(self.h - rock.y for rock in self.round_rocks)


def part1(input: str) -> int:
    return Platform.parse(input).tilted(Direction.N).weight()


def part2(input: str) -> int:
    return Platform.parse(input).cycled(1_000_000_000).weight()


TEST_INPUT = """O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."""

PART1_TESTS = [
    (TEST_INPUT, 136),
]

PART2_TESTS = [
    (TEST_INPUT, 64),
]
