from __future__ import annotations
from functools import cache
from dataclasses import dataclass
from multiprocessing import Pool


@dataclass(frozen=True)
class Point:
    x: int
    y: int
    z: int

    def parse(s: str) -> Point:
        return Point(*map(int, s.split(",")))


@dataclass(frozen=True)
class Brick:
    a: Point
    b: Point

    def parse(line: str) -> Brick:
        return Brick(*map(Point.parse, line.split("~")))

    def overlaps(self, other: Brick) -> bool:
        return (self.a.x <= other.a.x <= self.b.x or self.a.x <= other.b.x <= self.b.x or
                other.a.x <= self.a.x <= other.b.x or other.a.x <= self.b.x <= other.b.x) \
            and (self.a.y <= other.a.y <= self.b.y or self.a.y <= other.b.y <= self.b.y or
                 other.a.y <= self.a.y <= other.b.y or other.a.y <= self.b.y <= other.b.y)


@dataclass(frozen=True)
class State:
    bricks: tuple[Brick]

    def parse(input: str) -> State:
        return State(tuple(sorted(map(Brick.parse, input.splitlines()), key=lambda e: e.a.z)))

    def settled(self, dis: int | None = None) -> State | int:
        bricks, moved = list(self.bricks), 0

        if dis is not None:
            bricks.pop(dis)

        for i, u in enumerate(bricks):
            if (h := max((v.b.z + 1 for v in bricks[:i] if u.overlaps(v)), default=1)) < u.a.z:
                bricks[i] = Brick(
                    Point(u.a.x, u.a.y, h),
                    Point(u.b.x, u.b.y, h + u.b.z - u.a.z)
                )
                moved += 1

        return moved if dis is not None else State(tuple(bricks))

    @cache
    def solve(self) -> list[int]:
        # slow af, calculates by sheer force using a process pool
        with Pool() as pool:
            return pool.map(self.settled, range(len(self.bricks)))


def part1(input: str) -> int:
    return sum(m == 0 for m in State.parse(input).settled().solve())


def part2(input: str) -> int:
    return sum(State.parse(input).settled().solve())


TEST_INPUT = """1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9"""

PART1_TESTS = [
    (TEST_INPUT, 5),
]

PART2_TESTS = [
    (TEST_INPUT, 7),
]
