from typing import Callable


def process_lines(lines: list[str], condition: Callable) -> int:
    answer = 0

    for line in lines:
        a, b = line.split(",")
        a1, a2 = a.split("-")
        b1, b2 = b.split("-")

        if condition(int(a1), int(a2), int(b1), int(b2)):
            answer += 1

    return answer


def part1(input: str) -> int:
    def condition(a1, a2, b1, b2): return (a2 <= b2 and a1 >= b1) or (a2 >= b2 and a1 <= b1)
    return process_lines(input.splitlines(), condition)


def part2(input: str) -> int:
    def condition(a1, a2, b1, b2): return (a1 >= b1 and a1 <= b2) or (a2 >= b1 and a2 <= b2) or (b1 >= a1 and b1 <= a2) or (b2 >= a1 and b2 <= a2)
    return process_lines(input.splitlines(), condition)


TEST_INPUT = """2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"""

PART1_TESTS = [
    (TEST_INPUT, 2)
]

PART2_TESTS = [
    (TEST_INPUT, 4)
]
