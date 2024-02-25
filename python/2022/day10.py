def parse(input: str) -> dict[int, int]:
    lines = input.splitlines()
    data, cycle, register = dict(), 0, 1

    for line in lines:
        match line.strip().split(" "):
            case ["addx", value]:
                for _ in range(2):
                    cycle += 1
                    data[cycle] = register
                register += int(value)

            case ["noop"]:
                cycle += 1
                data[cycle] = register

    return data


def part1(input: str) -> int:
    data = parse(input)
    return sum(data[x] * x for x in range(20, 221, 40))


def part2(input: str) -> str:
    data = parse(input)
    height = 6
    width = 40
    state = [[False for _ in range(width)] for _ in range(height)]

    for key, value in data.items():
        sprite = [value - 1, value, value + 1]
        col = (key - 1) % width
        row = (key - col - 1) // width
        state[row][col] = col in sprite

    return "\n" + "\n".join("".join(["█" if col else " " for col in row]) for row in state) + "\n"


TEST_INPUT = """addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"""

PART1_TESTS = [
    (TEST_INPUT, 13140),
]

PART2_TESTS = [
    (TEST_INPUT, None),
]
