from __future__ import annotations


class Subset(tuple):
    def __new__(cls, *args) -> Subset:
        return tuple.__new__(cls, args)

    def parse(s: str) -> Subset:
        r, g, b = 0, 0, 0

        for splt in s.split(","):
            match splt.split():
                case [num, "red"]: r = int(num)
                case [num, "green"]: g = int(num)
                case [num, "blue"]: b = int(num)

        return Subset(r, g, b)

    @property
    def red(self):
        return self[0]

    @property
    def green(self):
        return self[1]

    @property
    def blue(self):
        return self[2]


class Game:
    id: int
    subsets: list[Subset]

    def __init__(self, line: str):
        splt = line.split(":")
        self.id = int(splt[0].split()[-1])
        self.subsets = [Subset.parse(s) for s in splt[1].split(";")]


def part1(input: str) -> int:
    lim = Subset(12, 13, 14)
    answer = 0

    for game in [Game(line) for line in input.splitlines()]:
        for ss in game.subsets:
            if ss.red > lim.red or ss.green > lim.green or ss.blue > lim.blue:
                break
        else:
            answer += game.id

    return answer


def part2(input: str) -> int:
    answer = 0

    for game in [Game(line) for line in input.splitlines()]:
        red = max(map(lambda ss: ss.red, game.subsets))
        green = max(map(lambda ss: ss.green, game.subsets))
        blue = max(map(lambda ss: ss.blue, game.subsets))
        answer += red * green * blue

    return answer


TEST_INPUT = """Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"""

PART1_TESTS = [
    (TEST_INPUT, 8),
]

PART2_TESTS = [
    (TEST_INPUT, 2286),
]
