from functools import reduce


def count_winners(time: int, distance: int) -> int:
    return sum((time - speed) * speed > distance for speed in range(1, time))


def part1(input: str) -> int:
    def parse_line(line: str) -> list[int]:
        return list(map(int, line.split(":")[1].split()))

    lines = input.splitlines()
    times, distances = parse_line(lines[0]), parse_line(lines[1])

    def f(acc: int, tup: tuple[int, int]) -> int:
        return acc * count_winners(tup[0], tup[1])

    return reduce(f, zip(times, distances), 1)


def part2(input: str) -> int:
    def parse_line(line: str) -> int:
        return int(line.split(":")[1].replace(" ", ""))

    lines = input.splitlines()
    time, distance = parse_line(lines[0]), parse_line(lines[1])

    return count_winners(time, distance)


TEST_INPUT = """Time:      7  15   30
Distance:  9  40  200"""

PART1_TESTS = [
    (TEST_INPUT, 288),
]

PART2_TESTS = [
    (TEST_INPUT, 71503),
]
