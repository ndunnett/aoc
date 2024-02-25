SHAPES = {
    "A": "rock",
    "B": "paper",
    "C": "scissors",
    "X": "rock",
    "Y": "paper",
    "Z": "scissors",
}

SHAPE_SCORES = {
    "rock": 1,
    "paper": 2,
    "scissors": 3,
}

SHAPE_DEFEATS = {
    "rock": "scissors",
    "paper": "rock",
    "scissors": "paper",
}

OUTCOMES = {
    "X": "lose",
    "Y": "draw",
    "Z": "win",
}


def part1(input: str) -> int:
    lines = input.splitlines()
    score = 0

    for line in lines:
        [opp, me] = line.split()
        opp = SHAPES[opp]
        me = SHAPES[me]
        score += SHAPE_SCORES[me]

        if opp == me:
            score += 3

        if SHAPE_DEFEATS[me] == opp:
            score += 6

    return score


def part2(input: str) -> int:
    lines = input.splitlines()
    score = 0

    for line in lines:
        [opp, me] = line.split()
        opp = SHAPES[opp]
        outcome = OUTCOMES[me]

        match outcome:
            case "lose":
                me = SHAPE_DEFEATS[opp]

            case "draw":
                score += 3
                me = opp

            case "win":
                score += 6
                me = SHAPE_DEFEATS[SHAPE_DEFEATS[opp]]

            case _:
                pass

        score += SHAPE_SCORES[me]

    return score


TEST_INPUT = """A Y
B X
C Z"""

PART1_TESTS = [
    (TEST_INPUT, 15)
]

PART2_TESTS = [
    (TEST_INPUT, 12)
]
