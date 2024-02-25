from itertools import pairwise


class Point(tuple):
    @property
    def x(self):
        return self[0]

    @property
    def y(self):
        return self[1]


class State:
    structure: set[Point[int, int]]
    sand: set[Point[int, int]]
    y_max: int
    x: int
    y: int

    def __init__(self, input: str):
        self.structure = set()

        for line in input.splitlines():
            points = [Point(map(int, point.split(","))) for point in line.split("->")]

            for start, end in pairwise(points):
                new = [start, end]

                if start.x != end.x:
                    new.extend(Point((x, start.y)) for x in range(min(start.x, end.x) + 1, max(start.x, end.x)))

                if start.y != end.y:
                    new.extend(Point((start.x, y)) for y in range(min(start.y, end.y) + 1, max(start.y, end.y)))

                self.structure.update(new)

        self.sand = self.structure.copy()
        self.y_max = max(point.y for point in self.structure)
        self.x, self.y = 500, 0

    def add_sand(self) -> None:
        self.sand.add((self.x, self.y - 1))
        self.x, self.y = 500, 0

    def solve(self, limit_y: bool) -> int:
        while (500, 0) not in self.sand and (not limit_y or self.y <= self.y_max):
            if (self.x, self.y) in self.sand:
                if (self.x - 1, self.y) not in self.sand:
                    self.x -= 1
                    continue
                elif (self.x + 1, self.y) not in self.sand:
                    self.x += 1
                    continue
                else:
                    self.add_sand()

            if not limit_y and self.y >= self.y_max + 2:
                self.add_sand()

            self.y += 1

        return len(self.sand.difference(self.structure))


def part1(input: str) -> int:
    return State(input).solve(True)


def part2(input: str) -> int:
    return State(input).solve(False)


TEST_INPUT = """498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"""

PART1_TESTS = [
    (TEST_INPUT, 24),
]

PART2_TESTS = [
    (TEST_INPUT, 93),
]
