SNAFU = {
    "=": -2,
    "-": -1,
    "0": 0,
    "1": 1,
    "2": 2
}


def part1(input: str) -> str:
    value = sum(SNAFU[d] * 5 ** p for line in input.splitlines() for p, d in enumerate(reversed(line)))
    answer = ""

    while value > 0:
        value += 2
        answer = list(SNAFU)[value % 5] + answer
        value //= 5

    return answer


TEST_INPUT = """1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122"""

PART1_TESTS = [
    (TEST_INPUT, "2=-1=0"),
]
