from __future__ import annotations
from functools import reduce
from itertools import batched


class Layer(tuple):
    def __new__(cls, dst: int, src: int, len: int) -> Layer:
        return tuple.__new__(cls, (dst, src, len))

    @property
    def dst(self) -> int:
        return self[0]

    @property
    def src(self) -> int:
        return self[1]

    @property
    def len(self) -> int:
        return self[2]


class Almanac:
    seeds: tuple[int]
    maps: list[list[Layer]]

    def __init__(self, input: str):
        sections = input.split("\n\n")
        self.seeds = tuple(map(int, sections[0].split(":")[1].split()))
        self.maps = [[Layer(*map(int, line.split())) for line in s.splitlines()[1:]] for s in sections[1:]]

    def transform(self, seed: int) -> int:
        def f(acc: int, map: list[Layer]) -> int:
            for layer in map:
                if layer.src <= acc < layer.src + layer.len:
                    return acc + layer.dst - layer.src
            return acc
        return reduce(f, self.maps, seed)

    def reverse_transform(self, location: int) -> int:
        def f(acc: int, map: list[Layer]) -> int:
            for layer in map:
                if layer.dst <= acc < layer.dst + layer.len:
                    return acc + layer.src - layer.dst
            return acc
        return reduce(f, reversed(self.maps), location)


def part1(input: str) -> int:
    a = Almanac(input)
    return min(a.transform(seed) for seed in a.seeds)


MAX_LOC = 100_000_000


def part2(input: str) -> int:
    a = Almanac(input)
    seed_ranges = [range(src, src + len) for src, len in batched(a.seeds, 2)]

    # slow
    for loc in range(MAX_LOC):
        seed = a.reverse_transform(loc)

        if any(map(lambda r: seed in r, seed_ranges)):
            return loc


TEST_INPUT = """seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"""

PART1_TESTS = [
    (TEST_INPUT, 35),
]

PART2_TESTS = [
    (TEST_INPUT, 46),
]
