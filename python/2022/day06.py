def get_unique_packet(signal: str, length: int) -> int:
    packet = []

    for i, char in enumerate(signal):
        packet.append(char)

        if len(packet) > length:
            packet = packet[-length:]

        if len(packet) == length and len(set(packet)) == length:
            return i + 1

    return 0


def part1(input: str) -> int:
    return get_unique_packet(input, 4)


def part2(input: str) -> int:
    return get_unique_packet(input, 14)


TEST_INPUT1 = "mjqjpqmgbljsphdztnvjfqwrcgsmlb"
TEST_INPUT2 = "bvwbjplbgvbhsrlpgdmjqwftvncz"
TEST_INPUT3 = "nppdvjthqldpwncqszvftbrmjlhg"
TEST_INPUT4 = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"
TEST_INPUT5 = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"

PART1_TESTS = [
    (TEST_INPUT1, 7),
    (TEST_INPUT2, 5),
    (TEST_INPUT3, 6),
    (TEST_INPUT4, 10),
    (TEST_INPUT5, 11),
]

PART2_TESTS = [
    (TEST_INPUT1, 19),
    (TEST_INPUT2, 23),
    (TEST_INPUT3, 23),
    (TEST_INPUT4, 29),
    (TEST_INPUT5, 26),
]
