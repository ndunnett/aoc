def replace_nums(input: str) -> str:
    return input \
        .replace("one", "o1e") \
        .replace("two", "t2o") \
        .replace("three", "t3e") \
        .replace("four", "f4r") \
        .replace("five", "f5e") \
        .replace("six", "s6x") \
        .replace("seven", "s7n") \
        .replace("eight", "e8t") \
        .replace("nine", "n9e")


def solve(line: str) -> int:
    res = ""

    for ch in line:
        if ch.isdigit():
            res += ch
            break

    for ch in reversed(line):
        if ch.isdigit():
            res += ch
            break

    return int(res)


def part1(input: str) -> int:
    return sum(solve(line) for line in input.splitlines())


def part2(input: str) -> int:
    return sum(solve(line) for line in replace_nums(input).splitlines())


TEST_INPUT1 = """1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"""

TEST_INPUT2 = """two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"""

PART1_TESTS = [
    (TEST_INPUT1, 142),
]

PART2_TESTS = [
    (TEST_INPUT2, 281),
]
