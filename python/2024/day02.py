from itertools import pairwise


def inspect_report(report: list[int]) -> bool:
    if report[0] < report[-1]:
        return all(a < b and b - a <= 3 for a, b in pairwise(report))
    else:
        return all(a > b and a - b <= 3 for a, b in pairwise(report))


class State:
    def __init__(self, input: str) -> None:
        self.reports = [list(map(int, line.split(" "))) for line in input.splitlines()]

    def safe_reports(self) -> int:
        return len(list(filter(inspect_report, self.reports)))

    def tolerable_reports(self) -> int:
        def f(report: list[int]) -> bool:
            return any(inspect_report(report[:i] + report[i + 1 :]) for i in range(len(report)))

        return len(list(filter(f, self.reports)))


def part1(input: str) -> int:
    return State(input).safe_reports()


def part2(input: str) -> int:
    return State(input).tolerable_reports()


TEST_INPUT = """7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"""

PART1_TESTS = [
    (TEST_INPUT, 2),
]

PART2_TESTS = [
    (TEST_INPUT, 4),
]
