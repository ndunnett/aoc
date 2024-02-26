from __future__ import annotations


class Point(tuple):
    @property
    def x(self) -> int:
        return self[0]

    @property
    def y(self) -> int:
        return self[1]

    @property
    def z(self) -> int:
        return self[2]

    def get_neighbours(self) -> list[Point]:
        return [
            Point((self.x + 1, self.y, self.z)),
            Point((self.x - 1, self.y, self.z)),
            Point((self.x, self.y + 1, self.z)),
            Point((self.x, self.y - 1, self.z)),
            Point((self.x, self.y, self.z + 1)),
            Point((self.x, self.y, self.z - 1))
        ]


class State:
    structure: set[Point]
    min: Point
    max: Point

    def __init__(self, input: str):
        self.structure = set(Point(map(int, cube.split(","))) for cube in input.splitlines())
        x_min, x_max = (x := sorted(p.x for p in self.structure))[0] - 1, x[-1] + 1
        y_min, y_max = (y := sorted(p.y for p in self.structure))[0] - 1, y[-1] + 1
        z_min, z_max = (z := sorted(p.z for p in self.structure))[0] - 1, z[-1] + 1
        self.min = Point((x_min, y_min, z_min))
        self.max = Point((x_max, y_max, z_max))

    def within_bounds(self, p: Point) -> bool:
        return self.min.x <= p.x <= self.max.x and self.min.y <= p.y <= self.max.y and self.min.z <= p.z <= self.max.z


def part1(input: str) -> int:
    s = State(input).structure
    return len([True for p in s for side in p.get_neighbours() if side not in s])


def part2(input: str) -> int:
    s = State(input)
    queue = [s.min]
    visited = set()
    answer = 0

    while queue and (next := queue.pop()):
        if next not in visited:
            visited.add(next)

            for p in filter(lambda p: p not in visited and s.within_bounds(p), next.get_neighbours()):
                if p in s.structure:
                    answer += 1
                else:
                    queue.append(p)

    return answer


TEST_INPUT = """2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5"""

PART1_TESTS = [
    (TEST_INPUT, 64),
]

PART2_TESTS = [
    (TEST_INPUT, 58),
]
