from ast import literal_eval
from typing import Any


def cmp(left: list[Any] | int, right: list[Any] | int) -> bool:
    if not isinstance(left, list) and not isinstance(right, list):
        return left < right

    if not isinstance(left, list):
        left = [left]

    if not isinstance(right, list):
        right = [right]

    for l, r in zip(left, right):
        if l != r:
            return cmp(l, r)

    return len(left) < len(right)


def part1(input: str) -> int:
    pairs = [map(literal_eval, pair.split("\n")) for pair in input.split("\n\n")]
    return sum(i + 1 for i, [left, right] in enumerate(pairs) if cmp(left, right))


def part2(input: str) -> int:
    packets = [literal_eval(line) for line in input.splitlines() if line] + [[[2]], [[6]]]
    two, six = None, None

    for i in range(len(packets)):
        for j in range(len(packets) - i - 1):
            if not cmp(packets[j], packets[j + 1]):
                packets[j], packets[j + 1] = packets[j + 1], packets[j]

        match (two, six, packets[-i]):
            case (None, _, [[2]]): two = len(packets) - i + 1
            case (_, None, [[6]]): six = len(packets) - i + 1
            case (None, _, _) | (_, None, _): pass
            case _: return two * six

    return None


TEST_INPUT = """[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"""

PART1_TESTS = [
    (TEST_INPUT, 13),
]

PART2_TESTS = [
    (TEST_INPUT, 140),
]
