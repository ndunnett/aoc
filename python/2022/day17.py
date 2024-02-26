SHAPES = {
    0: [0b0011110, 0b0000000, 0b0000000, 0b0000000],  # _
    1: [0b0001000, 0b0011100, 0b0001000, 0b0000000],  # +
    2: [0b0011100, 0b0000100, 0b0000100, 0b0000000],  # _|
    3: [0b0010000, 0b0010000, 0b0010000, 0b0010000],  # |
    4: [0b0011000, 0b0011000, 0b0000000, 0b0000000],  # []
}


def solve(stream: str, rocks: int) -> int:
    structure = []
    floor = [0] * 7
    stream_index = 0

    for rock in range(rocks):
        shape = SHAPES[rock % 5].copy()
        height = max(floor) + 5
        structure.extend([0b0000000] * (4 + height - len(structure)))
        at_rest = False

        while not at_rest:
            height -= 1
            collision = False

            if stream[stream_index] == "<":
                for i, line in enumerate(shape):
                    if line & 1 << 6 or line << 1 & structure[height + i]:
                        collision = True
                        break

                if not collision:
                    for i, line in enumerate(shape):
                        shape[i] <<= 1

            else:
                for i, line in enumerate(shape):
                    if line & 1 << 0 or line >> 1 & structure[height + i]:
                        collision = True
                        break

                if not collision:
                    for i, line in enumerate(shape):
                        shape[i] >>= 1

            stream_index = (stream_index + 1) % len(stream)

            if height <= 1:
                at_rest = True
            else:
                for i, line in enumerate(shape):
                    if line & structure[height + i - 1]:
                        at_rest = True
                        break

        for i, line in enumerate(shape):
            structure[i + height] |= line

            for j in range(7):
                if line & 1 << j:
                    floor[j] = max(floor[j], i + height)

    return max(floor)


def part1(input: str) -> int:
    return solve(input, 2022)


def part2(input: str) -> int:
    rocks = 1000000000000
    cycle_length = 5 * (len(input) + 1)

    while True:
        height = 0
        diff = []

        for i in range(6):
            new_height = solve(input, cycle_length * (i + 1))
            diff.append(height - new_height)
            height = new_height

        if sum(diff[1:]) == diff[1] * (len(diff) - 1):
            break

        cycle_length += 1

    remainder = rocks % cycle_length
    number_of_cycles = (rocks - remainder) // cycle_length
    return (solve(input, cycle_length * 2) - solve(input, cycle_length)) * number_of_cycles + solve(input, remainder)


TEST_INPUT = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"

PART1_TESTS = [
    (TEST_INPUT, 3068),
]

PART2_TESTS = [
    (TEST_INPUT, 1514285714288),
]
