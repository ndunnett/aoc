from __future__ import annotations
from lib import Point
from functools import reduce
from itertools import chain
from enum import Enum, auto
from collections import defaultdict


class Direction(Enum):
    N = auto()
    S = auto()
    E = auto()
    W = auto()


class Tile(Enum):
    Empty = auto()
    ForwardSlash = auto()
    BackSlash = auto()
    Horizontal = auto()
    Vertical = auto()

    def parse(ch: str) -> Tile:
        match ch:
            case "/": return Tile.ForwardSlash
            case "\\": return Tile.BackSlash
            case "-": return Tile.Horizontal
            case "|": return Tile.Vertical
            case _: return Tile.Empty


class Contraption(dict):
    def parse(input: str) -> Contraption:
        return Contraption({
            Point(x, y): Tile.parse(ch)
            for y, line in enumerate(input.splitlines())
            for x, ch in enumerate(line)
        })

    def trace_beam(self, position: Point, direction: Direction) -> set[Point]:
        beams = [(position, direction)]
        energised = defaultdict(set)

        def energise_tile(p: Point, d: Direction) -> bool:
            if p in self:
                if p in energised:
                    if d in energised[p]:
                        return False
                    else:
                        energised[p].add(d)
                        return True
                else:
                    energised[p].add(d)
                    return True
            else:
                return False

        while beams:
            p, d = beams.pop()

            while energise_tile(p, d):
                if p in self:
                    match self[p]:
                        case Tile.Vertical:
                            if d == Direction.E or d == Direction.W:
                                beams.append((p, Direction.N))
                                beams.append((p, Direction.S))
                                break

                        case Tile.Horizontal:
                            if d == Direction.N or d == Direction.S:
                                beams.append((p, Direction.E))
                                beams.append((p, Direction.W))
                                break

                        case Tile.ForwardSlash:
                            match d:
                                case Direction.N: d = Direction.E
                                case Direction.S: d = Direction.W
                                case Direction.E: d = Direction.N
                                case Direction.W: d = Direction.S

                        case Tile.BackSlash:
                            match d:
                                case Direction.N: d = Direction.W
                                case Direction.S: d = Direction.E
                                case Direction.E: d = Direction.S
                                case Direction.W: d = Direction.N

                        case Tile.Empty:
                            pass

                    match d:
                        case Direction.N: p += Point(0, -1)
                        case Direction.S: p += Point(0, 1)
                        case Direction.E: p += Point(1, 0)
                        case Direction.W: p += Point(-1, 0)

                else:
                    break

        return energised


def part1(input: str) -> int:
    return len(Contraption.parse(input).trace_beam(Point(0, 0), Direction.E))


def part2(input: str) -> int:
    c = Contraption.parse(input)
    max_x, max_y = reduce(lambda acc, p: (max(acc[0], p.x), max(acc[1], p.y)), c.keys(), (0, 0))

    def _solve(tup: tuple[Point, Direction]) -> int:
        return len(c.trace_beam(*tup))

    return max(map(_solve, chain(
        map(lambda x: (Point(x, 0), Direction.S), range(max_x + 1)),
        map(lambda x: (Point(x, max_y), Direction.N), range(max_x + 1)),
        map(lambda y: (Point(0, y), Direction.E), range(max_y + 1)),
        map(lambda y: (Point(max_x, y), Direction.W), range(max_y + 1)),
    )))


TEST_INPUT = r""".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."""

PART1_TESTS = [
    (TEST_INPUT, 46),
]

PART2_TESTS = [
    (TEST_INPUT, 51),
]
