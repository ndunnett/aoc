from __future__ import annotations
from lib import Point, get_limits
from functools import reduce, cache
from enum import Enum, auto


class Pipe(Enum):
    Cross = auto()
    Vertical = auto()
    Horizontal = auto()
    BendNE = auto()
    BendNW = auto()
    BendSW = auto()
    BendSE = auto()

    def parse(ch: str) -> Pipe | None:
        match ch:
            case "S": return Pipe.Cross
            case "|": return Pipe.Vertical
            case "-": return Pipe.Horizontal
            case "L": return Pipe.BendNE
            case "J": return Pipe.BendNW
            case "7": return Pipe.BendSW
            case "F": return Pipe.BendSE
            case _: return None

    @cache
    def offsets(self) -> list[Point]:
        match self:
            case Pipe.Vertical: return [Point(0, -1), Point(0, 1)]
            case Pipe.BendNE: return [Point(0, -1), Point(1, 0)]
            case Pipe.BendNW: return [Point(0, -1), Point(-1, 0)]
            case Pipe.BendSW: return [Point(0, 1), Point(-1, 0)]
            case Pipe.BendSE: return [Point(0, 1), Point(1, 0)]
            case Pipe.Horizontal: return [Point(1, 0), Point(-1, 0)]
            case Pipe.Cross: return [
                Point(-1, 0),
                Point(1, 0),
                Point(0, 1),
                Point(0, -1),
            ]


class Tile(tuple):
    @cache
    def __new__(cls, *args) -> Tile:
        return tuple.__new__(cls, args)

    @property
    def point(self) -> Point:
        return self[0]

    @property
    def pipe(self) -> Pipe:
        return self[1]


class Maze(dict):
    def parse(input: str) -> Maze:
        return Maze({
            Point(x, y): pipe
            for y, line in enumerate(input.splitlines())
            for x, ch in enumerate(line)
            if (pipe := Pipe.parse(ch)) is not None
        })

    def neighbours(self, tile: Tile) -> list[Tile]:
        return [Tile(p, self[p]) for off in tile.pipe.offsets() if (p := tile.point + off) in self]

    def connections(self, tile: Tile) -> list[Tile]:
        return [n for n in self.neighbours(tile) if tile in self.neighbours(n)]

    def reduced(self) -> Maze:
        for point, pipe in self.items():
            if pipe == Pipe.Cross:
                start = Tile(point, pipe)
                break

        seen = [start]
        queue = self.connections(start)

        while queue and (t := queue.pop()):
            if t.pipe == Pipe.Cross:
                break

            if t not in seen:
                queue.extend(self.connections(t))
                seen.append(t)

        return Maze(seen)

    def extent(self) -> tuple[Point, Point]:
        min_x, max_x = get_limits(map(lambda k: k.x, self.keys()))
        min_y, max_y = get_limits(map(lambda k: k.y, self.keys()))
        return (Point(min_x, min_y), Point(max_x, max_y))


def part1(input: str) -> int:
    return len(Maze.parse(input).reduced()) // 2


def part2(input: str) -> int:
    m = Maze.parse(input).reduced()
    _min, _max = m.extent()

    def _r(acc: tuple[int, bool], point: Point) -> int:
        match (acc[1], m[point] if point in m else None):
            case (_, Pipe.Vertical | Pipe.BendNW | Pipe.BendNE):
                return (acc[0], not acc[1])
            case (True, None):
                return (acc[0] + 1, acc[1])
            case _:
                return acc

    def _y(y: int) -> int:
        return reduce(_r, map(lambda x: Point(x, y), range(_min.x, _max.x + 1)), (0, False))[0]

    return sum(map(_y, range(_min.y, _max.y + 1)))


TEST_INPUT1 = """-L|F7
7S-7|
L|7||
-L-J|
L|-JF"""

TEST_INPUT2 = """..F7.
.FJ|.
SJ.L7
|F--J
LJ..."""

TEST_INPUT3 = """...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
..........."""

TEST_INPUT4 = """.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ..."""

TEST_INPUT5 = """FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L"""

PART1_TESTS = [
    (TEST_INPUT1, 4),
    (TEST_INPUT2, 8),
]

PART2_TESTS = [
    (TEST_INPUT3, 4),
    (TEST_INPUT4, 8),
    (TEST_INPUT5, 10),
]
