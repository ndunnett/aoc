def solve(input: str, iterations: int, key: int) -> int:
    encrypted = [(i, int(line) * key) for i, line in enumerate(input.splitlines())]
    mixed = encrypted.copy()
    zero = None

    for _ in range(iterations):
        for i, n in encrypted:
            current_index = mixed.index((i, n))
            mixed.pop(current_index)
            new_index = (current_index + n + len(mixed)) % len(mixed)
            mixed.insert(new_index, (i, n))

            if n == 0:
                zero = (i, n)

    return sum(mixed[(mixed.index(zero) + i) % len(mixed)][1] for i in [1000, 2000, 3000])


def part1(input: str) -> int:
    return solve(input, 1, 1)


def part2(input: str) -> int:
    return solve(input, 10, 811589153)


TEST_INPUT = """1
2
-3
3
-2
0
4"""

PART1_TESTS = [
    (TEST_INPUT, 3),
]

PART2_TESTS = [
    (TEST_INPUT, 1623178306),
]
