def solve(input: str, rev: bool = False) -> str:
    sections = input.split("\n\n")
    buckets = sections[0].splitlines()

    stacks = {
        n + 1: [buckets[j][i] for j in range(len(buckets) - 1, -1, -1) if buckets[j][i] != " "]
        for n, i in enumerate(range(1, len(buckets[-1]), 4))
    }

    for procedure in sections[1].splitlines():
        split = procedure.split()
        n, src, dst = int(split[1]), int(split[3]), int(split[5])
        to_move = [stacks[src].pop() for _ in range(0, n)]

        if rev:
            to_move = reversed(to_move)

        stacks[dst].extend(to_move)

    return "".join([crates[-1] for crates in stacks.values()])


def part1(input: str) -> str:
    return solve(input)


def part2(input: str) -> str:
    return solve(input, rev=True)


TEST_INPUT = """    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"""

PART1_TESTS = [
    (TEST_INPUT, "CMZ")
]

PART2_TESTS = [
    (TEST_INPUT, "MCD")
]
