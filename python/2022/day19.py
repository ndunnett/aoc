from __future__ import annotations
from functools import reduce
import re


RE = re.compile(r"^Blueprint (\d+): Each ore robot costs (\d+) ore\. Each clay robot costs (\d+) ore\. Each obsidian robot costs (\d+) ore and (\d+) clay\. Each geode robot costs (\d+) ore and (\d+) obsidian\.$")


class Blueprint(tuple):
    max: tuple[int, int, int]

    def __init__(self, *args, **kwargs):
        super().__init__()
        self.max = (max(self.ore_ore, self.clay_ore, self.obs_ore), self.obs_clay, self.geo_obs)

    @property
    def id(self) -> int:
        return self[0]

    @property
    def ore_ore(self) -> int:
        return self[1]

    @property
    def clay_ore(self) -> int:
        return self[2]

    @property
    def obs_ore(self) -> int:
        return self[3]

    @property
    def obs_clay(self) -> int:
        return self[4]

    @property
    def geo_ore(self) -> int:
        return self[5]

    @property
    def geo_obs(self) -> int:
        return self[6]


class State(tuple):
    def make_ore_bot(self, bp: Blueprint) -> State:
        return State((
            self.time - 1, self.ore_bots + 1, self.clay_bots, self.obs_bots, self.geo_bots,
            self.ore + self.ore_bots - bp.ore_ore,
            self.clay + self.clay_bots,
            self.obs + self.obs_bots,
            self.geo + self.geo_bots
        ))

    def make_clay_bot(self, bp: Blueprint) -> State:
        return State((
            self.time - 1, self.ore_bots, self.clay_bots + 1, self.obs_bots, self.geo_bots,
            self.ore + self.ore_bots - bp.clay_ore,
            self.clay + self.clay_bots,
            self.obs + self.obs_bots,
            self.geo + self.geo_bots
        ))

    def make_obs_bot(self, bp: Blueprint) -> State:
        return State((
            self.time - 1, self.ore_bots, self.clay_bots, self.obs_bots + 1, self.geo_bots,
            self.ore + self.ore_bots - bp.obs_ore,
            self.clay + self.clay_bots - bp.obs_clay,
            self.obs + self.obs_bots,
            self.geo + self.geo_bots
        ))

    def make_geo_bot(self, bp: Blueprint) -> State:
        return State((
            self.time - 1, self.ore_bots, self.clay_bots, self.obs_bots, self.geo_bots + 1,
            self.ore + self.ore_bots - bp.geo_ore,
            self.clay + self.clay_bots,
            self.obs + self.obs_bots - bp.geo_obs,
            self.geo + self.geo_bots
        ))

    def accumulate(self) -> State:
        return State((
            self.time - 1, self.ore_bots, self.clay_bots, self.obs_bots, self.geo_bots,
            self.ore + self.ore_bots,
            self.clay + self.clay_bots,
            self.obs + self.obs_bots,
            self.geo + self.geo_bots
        ))

    def optimise(self, bp: Blueprint) -> State:
        return State((
            self.time, self.ore_bots, self.clay_bots, self.obs_bots, self.geo_bots,
            min(self.ore, self.time * bp.max[0] - self.ore_bots * (self.time - 1)),
            min(self.clay, self.time * bp.max[1] - self.clay_bots * (self.time - 1)),
            min(self.obs, self.time * bp.max[2] - self.obs_bots * (self.time - 1)),
            self.geo
        ))

    @property
    def key(self) -> tuple[int, int, int, int, int]:
        return self[:5]

    @property
    def value(self) -> tuple[int, int, int, int]:
        return self[5:]

    @property
    def time(self) -> int:
        return self[0]

    @property
    def ore_bots(self) -> int:
        return self[1]

    @property
    def clay_bots(self) -> int:
        return self[2]

    @property
    def obs_bots(self) -> int:
        return self[3]

    @property
    def geo_bots(self) -> int:
        return self[4]

    @property
    def ore(self) -> int:
        return self[5]

    @property
    def clay(self) -> int:
        return self[6]

    @property
    def obs(self) -> int:
        return self[7]

    @property
    def geo(self) -> int:
        return self[8]


def parse(input: str) -> list[Blueprint]:
    return [Blueprint(map(int, RE.search(line).groups())) for line in input.splitlines()]


def solve(bp: Blueprint, time: int) -> int:
    queue = [State((time, 1, 0, 0, 0, 0, 0, 0, 0))]
    visited = dict()
    answer = 0

    while queue and (s := queue.pop().optimise(bp)):
        if s.key in visited:
            if all(a <= b for a, b in zip(s.value, visited[s.key])):
                continue

        visited[s.key] = s.value

        if s.time == 0:
            answer = max(s.geo, answer)
            continue

        max_ore, max_clay, max_obs = bp.max

        if s.ore < max_ore * 1.25:
            queue.append(s.accumulate())

        if s.ore >= bp.ore_ore and s.geo_bots < 1 and s.ore_bots < max_ore:
            queue.append(s.make_ore_bot(bp))

        if s.ore >= bp.clay_ore and s.geo_bots < 1 and s.clay_bots < max_clay:
            queue.append(s.make_clay_bot(bp))

        if s.ore >= bp.obs_ore and s.clay >= bp.obs_clay and s.obs_bots < max_obs:
            queue.append(s.make_obs_bot(bp))

        if s.ore >= bp.geo_ore and s.obs >= bp.geo_obs:
            queue.append(s.make_geo_bot(bp))

    return answer


def part1(input: str) -> int:
    return sum(bp.id * solve(bp, 24) for bp in parse(input))


def part2(input: str) -> int:
    return reduce(lambda a, b: a * b, [solve(bp, 32) for bp in parse(input)[:3]])


TEST_INPUT = """Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian."""

PART1_TESTS = [
    (TEST_INPUT, 33),
]

PART2_TESTS = [
    (TEST_INPUT, 56 * 62),
]
