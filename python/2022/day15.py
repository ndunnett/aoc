import re
from functools import cache


RE = re.compile(r"\-?\d+")


def parse(input: str) -> list[tuple[int, int, int, int]]:
    return sorted(tuple(map(int, RE.findall(line))) for line in input.splitlines())


@cache
def distance(ax: int, ay: int, bx: int, by: int) -> int:
    return abs(ax - bx) + abs(ay - by)


def part1(input: str) -> int:
    sensors = parse(input)
    row = 2000000 if len(sensors) > 14 else 10
    coverage = set()
    beacons = set()

    for sx, sy, bx, by in sensors:
        dist = distance(sx, sy, bx, by)

        if (width := dist - abs(row - sy)) > 0:
            coverage.update(range(sx - width, sx + width + 1))

        if by == row:
            beacons.add(bx)

    return len(coverage.difference(beacons))


def part2(input: str) -> int:
    sensors = parse(input)
    y_limit = 4000000 if len(sensors) > 14 else 20

    for y in range(y_limit + 1):
        ranges = sorted(
            (sx - dist, sx + dist)
            for sx, sy, bx, by in sensors
            if (dist := distance(sx, sy, bx, by) - abs(y - sy)) >= 0
        )

        for x_low, x_high in ranges[1:]:
            if x_low <= ranges[0][1]:
                ranges[0] = (ranges[0][0], max(ranges[0][1], x_high))
            else:
                return (ranges[0][1] + 1) * 4000000 + y

    return None


TEST_INPUT = """Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3"""

PART1_TESTS = [
    (TEST_INPUT, 26),
]

PART2_TESTS = [
    (TEST_INPUT, 56000011),
]
