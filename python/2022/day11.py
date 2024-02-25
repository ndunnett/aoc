from functools import reduce


class Monkey:
    def __init__(self, input: str):
        self.inspections = 0

        for split in [line.strip().split(":") for line in input.splitlines()]:
            match split[0].strip():
                case "Starting items":
                    self.items = [int(item.strip()) for item in split[1].split(",")]
                case "Operation":
                    self.operation = eval("lambda old: " + split[1].split("=")[1].strip())
                case "Test":
                    self.test = int(split[1].split("divisible by")[-1].strip())
                case "If true":
                    self.true = int(split[1].split("throw to monkey")[-1].strip())
                case "If false":
                    self.false = int(split[1].split("throw to monkey")[-1].strip())


def solve(input: str, rounds: int, divisor: int | None, mod: bool) -> list[Monkey]:
    monkeys = [Monkey(section) for section in input.split("\n\n")]
    modulus = reduce(lambda x, y: x * y, (monkey.test for monkey in monkeys)) if mod else None

    for _ in range(rounds):
        for monkey in monkeys:
            for item in monkey.items:
                worry = monkey.operation(item)

                if modulus:
                    worry %= modulus

                if divisor:
                    worry //= divisor

                monkeys[monkey.true if worry % monkey.test == 0 else monkey.false].items.append(worry)

            monkey.inspections += len(monkey.items)
            monkey.items.clear()

    inspections = sorted((monkey.inspections for monkey in monkeys), reverse=True)
    return inspections[0] * inspections[1]


def part1(input: str) -> int:
    return solve(input, 20, 3, False)


def part2(input: str) -> int:
    return solve(input, 10000, None, True)


TEST_INPUT = """Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1"""

PART1_TESTS = [
    (TEST_INPUT, 10605),
]

PART2_TESTS = [
    (TEST_INPUT, 2713310158),
]
