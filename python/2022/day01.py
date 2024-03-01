def process(input: str) -> list[int]:
    return sorted([sum(map(int, r.splitlines())) for r in input.split("\n\n")], reverse=True)


def part1(input: str) -> int:
    return process(input)[0]


def part2(input: str) -> int:
    return sum(process(input)[:3])


TEST_INPUT = """1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"""

PART1_TESTS = [
    (TEST_INPUT, 24000)
]

PART2_TESTS = [
    (TEST_INPUT, 45000)
]
