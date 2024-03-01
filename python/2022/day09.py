from __future__ import annotations


def sign(x: int) -> int:
    return 1 if x > 0 else (-1 if x < 0 else 0)


class Knot:
    x: int
    y: int
    visited: set[tuple[int, int]]
    follower: Knot | None

    def __init__(self, x: int, y: int):
        self.x, self.y = x, y
        self.visited = set((x, y))
        self.follower = None

    @property
    def tail(self) -> Knot:
        return self.follower.tail if self.follower else self

    def add_follower(self) -> None:
        self.tail.follower = Knot(self.x, self.y)

    def move(self, x: int, y: int) -> None:
        self.x += x
        self.y += y

        if self.follower:
            while not (-1 <= self.x - self.follower.x <= 1 and -1 <= self.y - self.follower.y <= 1):
                self.follower.move(sign(self.x - self.follower.x), sign(self.y - self.follower.y))
        else:
            self.visited.add((self.x, self.y))

    def command(self, line: str) -> None:
        match line.split(" "):
            case ["U", value]:
                self.move(0, int(value))
            case ["D", value]:
                self.move(0, -int(value))
            case ["R", value]:
                self.move(int(value), 0)
            case ["L", value]:
                self.move(-int(value), 0)


def solve(lines: list[str], followers: int) -> int:
    head = Knot(0, 0)

    for _ in range(followers):
        head.add_follower()

    for line in lines:
        head.command(line)

    return len(head.tail.visited)


def part1(input: str) -> int:
    return solve(input.splitlines(), 1)


def part2(input: str) -> int:
    return solve(input.splitlines(), 9)


TEST_INPUT1 = """R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"""

TEST_INPUT2 = """R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"""

PART1_TESTS = [
    (TEST_INPUT1, 13),
]

PART2_TESTS = [
    (TEST_INPUT1, 1),
    (TEST_INPUT2, 36),
]
