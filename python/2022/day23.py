from __future__ import annotations
from collections import defaultdict
from functools import cache


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


N = Point(0, -1)
S = Point(0, 1)
W = Point(-1, 0)
E = Point(1, 0)

MOVESETS = (
    N + W, N, N + E,
    S + W, S, S + E,
    W + N, W, W + S,
    E + N, E, E + S,
)

DIRECTIONS = (
    N + W, N, N + E,
    W, E,
    S + W, S, S + E
)


class State(set):
    round: int

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.round = 0

    def parse(input: str) -> State:
        return State([
            Point(x, y)
            for y, line in enumerate(input.splitlines())
            for x, element in enumerate(line)
            if element == "#"
        ])

    def move_elves(self) -> bool:
        proposed_moves = defaultdict(lambda: [])

        for elf in self:
            for move in DIRECTIONS:
                if elf + move in self:
                    break
            else:
                continue

            for i in range(4):
                index = ((self.round + i) % 4) * 3
                moves = [elf + move for move in MOVESETS[index:index + 3]]

                if self.isdisjoint(moves):
                    proposed_moves[moves[1]].append(elf)
                    break

        moved = False

        for move, elves in proposed_moves.items():
            if len(elves) == 1:
                self.discard(elves[0])
                self.add(move)
                moved = True

        self.round += 1
        return moved


def part1(input: str) -> int:
    elves = State.parse(input)

    for _ in range(10):
        elves.move_elves()

    x_min, x_max = (x := sorted(elf.x for elf in elves))[0], x[-1]
    y_min, y_max = (y := sorted(elf.y for elf in elves))[0], y[-1]
    return (x_max - x_min + 1) * (y_max - y_min + 1) - len(elves)


def part2(input: str) -> int:
    elves = State.parse(input)

    while elves.move_elves():
        pass

    return elves.round


TEST_INPUT = """....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#.."""

PART1_TESTS = [
    (TEST_INPUT, 110),
]

PART2_TESTS = [
    (TEST_INPUT, 20),
]
