from typing import Any


def parse(input: str) -> list[list[int]]:
    return [[int(ch) for ch in line.strip()] for line in input.splitlines()]


def rotate(matrix: list[list[Any]]) -> list[list[Any]]:
    return [[matrix[len(row) - 1 - j][i] for j, _ in enumerate(row)] for i, row in enumerate(matrix)]


def part1(input: str) -> int:
    trees = parse(input)
    visibility = [[False for _ in range(len(trees[0]))] for _ in range(len(trees))]

    for _ in range(4):
        for i, row in enumerate(trees):
            current = -1
            for j, col in enumerate(row):
                if col > current:
                    current = col
                    visibility[i][j] = True

        trees = rotate(trees)
        visibility = rotate(visibility)

    return sum(int(item) for list in visibility for item in list)


def part2(input: str) -> int:
    trees = parse(input)
    rows = len(trees)
    cols = len(trees[0])

    def scenic_score(x: int, y: int, tree: int) -> int:
        down, up, right, left = 0, 0, 0, 0

        for _y in range(y + 1, rows):
            down += 1
            if trees[_y][x] >= tree:
                break

        for _y in range(y - 1, -1, -1):
            up += 1
            if trees[_y][x] >= tree:
                break

        for _x in range(x + 1, cols):
            right += 1
            if trees[y][_x] >= tree:
                break

        for _x in range(x - 1, -1, -1):
            left += 1
            if trees[y][_x] >= tree:
                break

        return down * up * right * left

    return max(scenic_score(x, y, tree) for y, row in enumerate(trees) for x, tree in enumerate(row))


TEST_INPUT = """30373
25512
65332
33549
35390"""

PART1_TESTS = [
    (TEST_INPUT, 21),
]

PART2_TESTS = [
    (TEST_INPUT, 8),
]
