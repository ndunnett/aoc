from __future__ import annotations

from collections import defaultdict
from itertools import combinations
from typing import TYPE_CHECKING

from lib import Point

if TYPE_CHECKING:
    from collections.abc import Iterable


class Frequencies:
    freq_groups: defaultdict[str, list[Point]]
    size: int

    def __init__(self, input: str) -> None:
        self.freq_groups = defaultdict(lambda: [])

        for y, line in enumerate(input.splitlines()):
            for x, ch in enumerate(line):
                if ch != ".":
                    self.freq_groups[ch].append(Point(x, y))

        self.size = len(input.splitlines())

    def in_range(self, p: Point) -> bool:
        return 0 <= p.x < self.size and 0 <= p.y < self.size

    def groups(self) -> Iterable[list[Point]]:
        return self.freq_groups.values()


def part1(input: str) -> int:
    frequencies = Frequencies(input)
    antinodes = set()

    for frequency in frequencies.groups():
        for a, b in combinations(frequency, 2):
            antinode_a = a * 2 - b
            antinode_b = b * 2 - a

            if frequencies.in_range(antinode_a):
                antinodes.add(antinode_a)

            if frequencies.in_range(antinode_b):
                antinodes.add(antinode_b)

    return len(antinodes)


def part2(input: str) -> int:
    frequencies = Frequencies(input)
    antinodes = set()

    for frequency in frequencies.groups():
        for a, b in combinations(frequency, 2):
            antinode = a
            d = b - a

            while frequencies.in_range(antinode):
                antinodes.add(antinode)
                antinode = antinode - d

            antinode = a + d

            while frequencies.in_range(antinode):
                antinodes.add(antinode)
                antinode = antinode + d

    return len(antinodes)


TEST_INPUT = """............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
"""

PART1_TESTS = [
    (TEST_INPUT, 14),
]

PART2_TESTS = [
    (TEST_INPUT, 34),
]
