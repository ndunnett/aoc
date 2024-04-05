from __future__ import annotations
from lib import Point
from enum import Enum, auto
from collections import defaultdict
from dataclasses import dataclass


class Direction(Enum):
    Up = auto()
    Down = auto()
    Left = auto()
    Right = auto()

    def move(self) -> Point:
        match self:
            case Direction.Up: return Point(0, -1)
            case Direction.Down: return Point(0, 1)
            case Direction.Left: return Point(-1, 0)
            case Direction.Right: return Point(1, 0)

    def all() -> list[Direction]:
        return [Direction.Up, Direction.Down, Direction.Left, Direction.Right]


class Tile(Enum):
    Open = auto()
    Forest = auto()
    SlopeUp = auto()
    SlopeDown = auto()
    SlopeLeft = auto()
    SlopeRight = auto()

    def parse(ch: str) -> Tile | None:
        match ch:
            case ".": return Tile.Open
            case "^": return Tile.SlopeUp
            case "v": return Tile.SlopeDown
            case "<": return Tile.SlopeLeft
            case ">": return Tile.SlopeRight
            case _: return None


@dataclass(frozen=True)
class State:
    start: Point
    finish: Point
    graph: dict[Point, set[tuple[Point, int]]]

    @classmethod
    def parse(cls, input: str) -> State:
        map = {
            Point(x, y): tile
            for y, line in enumerate(input.splitlines())
            for x, ch in enumerate(line)
            if (tile := Tile.parse(ch))
        }

        keys = list(map.keys())
        return cls(keys[0], keys[-1], cls.make_graph(map))

    def make_graph(map: dict[Point, Tile]) -> dict[Point, set[tuple[Point, int]]]:
        raise NotImplementedError()

    def solve(self) -> int:
        queue, answer = [(self.start, {self.start}, 0)], 0

        while queue:
            pos, seen, steps = queue.pop()

            if pos == self.finish:
                if steps > answer:
                    answer = steps
                continue

            seen.add(pos)
            queue.extend(
                (np, seen | {pos}, steps + cost)
                for np, cost in self.graph[pos]
                if np not in seen
            )

        return answer


class StateSlippery(State):
    def make_graph(map: dict[Point, Tile]) -> dict[Point, set[tuple[Point, int]]]:
        def can_traverse(tile: Tile, dir: Direction) -> bool:
            match (tile, dir):
                case (Tile.Open, _): return True
                case (Tile.SlopeUp, Direction.Up): return True
                case (Tile.SlopeDown, Direction.Down): return True
                case (Tile.SlopeLeft, Direction.Left): return True
                case (Tile.SlopeRight, Direction.Right): return True
                case _: return False

        return {
            pos: {
                (np, 1) for dir in Direction.all()
                if (np := pos + dir.move()) in map and can_traverse(tile, dir)
            } for pos, tile in map.items()
        }


class StateNormal(State):
    def make_graph(map: dict[Point, Tile]) -> dict[Point, set[tuple[Point, int]]]:
        raw_graph = {
            pos: {np for dir in Direction.all() if (np := pos + dir.move()) in map}
            for pos in map
        }

        stack = [list(map.keys())[0]]
        seen = set()
        sparse_graph = defaultdict(set)

        while stack and (node := stack.pop()):
            seen.add(node)

            for next_node in raw_graph[node]:
                if next_node not in seen:
                    cost = 1

                    while next_node not in seen and len(raw_graph[next_node]) == 2:
                        seen.add(next_node)
                        next_node = next(n for n in raw_graph[next_node] if n not in seen)
                        cost += 1

                    stack.append(next_node)
                    sparse_graph[node].add((next_node, cost))
                    sparse_graph[next_node].add((node, cost))

        return sparse_graph


def part1(input: str) -> int:
    return StateSlippery.parse(input).solve()


def part2(input: str) -> int:
    return StateNormal.parse(input).solve()


TEST_INPUT = """#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#"""

PART1_TESTS = [
    (TEST_INPUT, 94),
]

PART2_TESTS = [
    (TEST_INPUT, 154),
]
