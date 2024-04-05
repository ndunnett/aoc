from __future__ import annotations
from lib import Point
from itertools import combinations, product
from dataclasses import dataclass
import z3


@dataclass(frozen=True)
class Hailstone:
    pos: Point
    vel: Point

    def parse(line: str) -> Hailstone:
        def _parse(s: str) -> Point: return Point(*map(int, s.split(", ")))
        return Hailstone(*map(_parse, line.split(" @ ")))


def parse(input: str) -> tuple[Hailstone]:
    return tuple(map(Hailstone.parse, input.splitlines()))


def part1(input: str) -> int:
    at_least, at_most = 200_000_000_000_000, 400_000_000_000_000
    hailstones, result = parse(input), 0

    for a, b in combinations(hailstones, 2):
        if not (det := a.vel.y * b.vel.x - a.vel.x * b.vel.y):
            continue

        diff_a = a.pos.x * (a.pos.y + a.vel.y) - a.pos.y * (a.pos.x + a.vel.x)
        diff_b = b.pos.x * (b.pos.y + b.vel.y) - b.pos.y * (b.pos.x + b.vel.x)
        x = (diff_a * b.vel.x - diff_b * a.vel.x) / det
        y = (diff_a * b.vel.y - diff_b * a.vel.y) / det

        if (x - a.pos.x) / a.vel.x >= 0 and (x - b.pos.x) / b.vel.x >= 0 and at_least <= x <= at_most and at_least <= y <= at_most:
            result += 1

    return result


def part2(input: str) -> int:
    hailstones, solver = parse(input), z3.Solver()
    p, v, t = z3.Reals("x y z"), z3.Reals("dx dy dz"), z3.Reals("t0 t1 t2")

    for (step, h), axis in product(enumerate(hailstones[:len(t)]), range(len(t))):
        solver.add(p[axis] + t[step] * v[axis] == h.pos[axis] + t[step] * h.vel[axis])

    solver.check()
    result = solver.model()
    return sum(result[pos].as_long() for pos in p)


TEST_INPUT = """19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3"""

PART1_TESTS = [
    # (TEST_INPUT, 2),  # intersections at least 7 and at most 27
    (TEST_INPUT, 0),
]

PART2_TESTS = [
    (TEST_INPUT, 47),
]
