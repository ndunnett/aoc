from __future__ import annotations

from enum import Enum, auto
from math import sqrt
from typing import TYPE_CHECKING, Self

if TYPE_CHECKING:
    from collections.abc import Generator


class GridIterationMethod(Enum):
    Horizontal = auto()
    Vertical = auto()
    DiagonalUp = auto()
    DiagonalDown = auto()
    Cross = auto()


class Grid:
    def __init__(self, input: str) -> None:
        self.letters = "".join(input.splitlines())
        self.size = int(sqrt(len(self.letters)))
        self.method = GridIterationMethod.Horizontal
        self.index = self.horizontal_index()

    def cross(self) -> Self:
        self.method = GridIterationMethod.Cross
        self.index = self.cross_index()
        return self

    def horizontal_index(self) -> Generator[tuple[int, int, int, int] | None]:
        for x in range(0, self.size - 3):
            for y in range(0, self.size):
                if self.letters[x + y * self.size] in ("X", "S"):
                    yield (
                        x + y * self.size,
                        x + 1 + y * self.size,
                        x + 2 + y * self.size,
                        x + 3 + y * self.size,
                    )
        yield None

    def vertical_index(self) -> Generator[tuple[int, int, int, int] | None]:
        for x in range(0, self.size):
            for y in range(0, self.size - 3):
                if self.letters[x + y * self.size] in ("X", "S"):
                    yield (
                        x + y * self.size,
                        x + (y + 1) * self.size,
                        x + (y + 2) * self.size,
                        x + (y + 3) * self.size,
                    )
        yield None

    def diagonal_up_index(self) -> Generator[tuple[int, int, int, int] | None]:
        for x in range(0, self.size - 3):
            for y in range(3, self.size):
                if self.letters[x + y * self.size] in ("X", "S"):
                    yield (
                        x + y * self.size,
                        x + 1 + (y - 1) * self.size,
                        x + 2 + (y - 2) * self.size,
                        x + 3 + (y - 3) * self.size,
                    )
        yield None

    def diagonal_down_index(self) -> Generator[tuple[int, int, int, int] | None]:
        for x in range(0, self.size - 3):
            for y in range(0, self.size - 3):
                if self.letters[x + y * self.size] in ("X", "S"):
                    yield (
                        x + y * self.size,
                        x + 1 + (y + 1) * self.size,
                        x + 2 + (y + 2) * self.size,
                        x + 3 + (y + 3) * self.size,
                    )
        yield None

    def cross_index(self) -> Generator[tuple[int, int, int, int] | None]:
        for x in range(0, self.size - 2):
            for y in range(0, self.size - 2):
                if self.letters[x + 1 + (y + 1) * self.size] == "A":
                    yield (
                        x + y * self.size,
                        x + 2 + y * self.size,
                        x + 2 + (y + 2) * self.size,
                        x + (y + 2) * self.size,
                    )
        yield None

    def __iter__(self) -> Self:
        return self

    def __next__(self) -> tuple[str, str, str, str]:
        index = next(self.index)

        if index is None:
            match self.method:
                case GridIterationMethod.Horizontal:
                    self.method = GridIterationMethod.Vertical
                    self.index = self.vertical_index()
                case GridIterationMethod.Vertical:
                    self.method = GridIterationMethod.DiagonalUp
                    self.index = self.diagonal_up_index()
                case GridIterationMethod.DiagonalUp:
                    self.method = GridIterationMethod.DiagonalDown
                    self.index = self.diagonal_down_index()
                case GridIterationMethod.DiagonalDown | GridIterationMethod.Cross:
                    raise StopIteration

            index = next(self.index)

        match index:
            case (a, b, c, d):
                return (
                    self.letters[a],
                    self.letters[b],
                    self.letters[c],
                    self.letters[d],
                )
            case _:
                raise StopIteration


def part1(input: str) -> int:
    def f(tup: tuple[str, str, str, str]) -> bool:
        match tup:
            case ("X", "M", "A", "S") | ("S", "A", "M", "X"):
                return True
            case _:
                return False

    return len(list(filter(f, Grid(input))))


def part2(input: str) -> int:
    def f(tup: tuple[str, str, str, str]) -> bool:
        match tup:
            case (
                ("M", "M", "S", "S")
                | ("M", "S", "S", "M")
                | ("S", "S", "M", "M")
                | ("S", "M", "M", "S")
            ):
                return True
            case _:
                return False

    return len(list(filter(f, Grid(input).cross())))


TEST_INPUT = """MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"""

PART1_TESTS = [
    (TEST_INPUT, 18),
]

PART2_TESTS = [
    (TEST_INPUT, 9),
]
