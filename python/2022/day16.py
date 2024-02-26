from functools import cache


class State:
    rates: dict[str, int]
    neighbours: dict[str, set[str]]

    def __init__(self, input: str):
        lines = [line.split() for line in input.splitlines()]
        self.rates = {s[1]: rate for s in lines if (rate := int(s[4][5:-1])) > 0}
        self.neighbours = {s[1]: set("".join(s[9:]).split(",")) for s in lines}
        self.time = 0

    @property
    @cache
    def valves(self):
        return list(self.neighbours.keys())

    @cache
    def bfs(self, src: str, dst: str) -> int:
        visited = set(src)
        queue = [(src, 0)]

        while queue:
            v, t = queue.pop()

            for n in self.neighbours[v]:
                if n not in visited:
                    if n == dst:
                        return t + 1

                    queue = [(n, t + 1)] + queue
                    visited.add(n)

        return None

    @cache
    def valve_index(self, valve: str) -> int:
        return sum(2 ** i for i, v in enumerate(self.rates.keys()) if v == valve)

    def find_paths(self, last_valve: str, opened: int, flow: int, time: int, result: dict[int, int]) -> dict[int, int]:
        if opened in result:
            result[opened] = max(result[opened], flow)
        else:
            result[opened] = flow

        for valve, rate in self.rates.items():
            if opened & self.valve_index(valve) or valve == last_valve:
                continue

            time_left = time - self.bfs(last_valve, valve) - 1

            if time_left > 0:
                self.find_paths(valve, opened | self.valve_index(valve), flow + rate * time_left, time_left, result)

        return result


def part1(input: str) -> int:
    return max(State(input).find_paths("AA", 0, 0, 30, {}).values())


def part2(input: str) -> int:
    paths = list(filter(lambda i: i[0] >= 26 and i[1] >= 750, State(input).find_paths("AA", 0, 0, 26, {}).items()))
    return max(f1 + f2 for s1, f1 in paths for s2, f2 in paths if not s1 & s2)


TEST_INPUT = """Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II"""

PART1_TESTS = [
    (TEST_INPUT, 1651),
]

PART2_TESTS = [
    (TEST_INPUT, 1707),
]
