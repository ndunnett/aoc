from __future__ import annotations
from lib import Point, get_limits
from functools import cache, cached_property, reduce
from itertools import pairwise
from enum import Enum, auto
import operator


class Axis(Enum):
    V = auto()
    H = auto()

    def inverse(self) -> Axis:
        match self:
            case Axis.V: return Axis.H
            case Axis.H: return Axis.V


class Pt(Point):
    def pivoted(self, axis: Axis) -> Pt:
        match axis:
            case Axis.V: return self
            case Axis.H: return Pt(self.y, self.x)

    def view(self, axis: Axis) -> int:
        match axis:
            case Axis.V: return self.x
            case Axis.H: return self.y


class MirrorView(tuple):
    def __new__(cls, args) -> MirrorView:
        return tuple.__new__(cls, args)

    @cache
    def find_reflection(self, smudges: int) -> int | None:
        def is_perfect_reflection(r: int) -> bool:
            return sum(bin(a ^ b).count("1") for a, b in zip(reversed(self[:r]), self[r:])) == smudges

        for i, (ra, rb) in enumerate(pairwise(self)):
            if bin(ra ^ rb).count("1") <= smudges and is_perfect_reflection(i + 1):
                return i + 1

        return None


class Mirror(tuple):
    def __new__(cls, args) -> Mirror:
        return tuple.__new__(cls, args)

    @cache
    def parse(chunk: str) -> Mirror:
        return Mirror(
            Pt(x, y)
            for y, line in enumerate(chunk.splitlines())
            for x, ch in enumerate(line)
            if ch == "#"
        )

    @cached_property
    def extent(self) -> Pt:
        return Pt(
            get_limits(map(lambda k: k.x, self))[1],
            get_limits(map(lambda k: k.y, self))[1],
        )

    @cache
    def view(self, axis: Axis) -> MirrorView:
        return MirrorView(
            reduce(operator.or_, (
                1 << b
                for b in range(self.extent.view(axis.inverse()) + 1)
                if Pt(a, b).pivoted(axis) in self
            ))
            for a in range(self.extent.view(axis) + 1)
        )


def solve(input: str, smudges: int) -> int:
    def _solve(chunk: str) -> int:
        m = Mirror.parse(chunk)
        if v := m.view(Axis.V).find_reflection(smudges):
            return v
        elif h := m.view(Axis.H).find_reflection(smudges):
            return h * 100
        else:
            return 0

    return sum(map(_solve, input.split("\n\n")))


def part1(input: str) -> int:
    return solve(input, 0)


def part2(input: str) -> int:
    return solve(input, 1)


TEST_INPUT = """#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"""

PART1_TESTS = [
    (TEST_INPUT, 405),
]

PART2_TESTS = [
    (TEST_INPUT, 400),
]
