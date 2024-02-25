def get_priority(letter: str) -> int:
    if letter.islower():
        return ord(letter) - ord("a") + 1
    else:
        return ord(letter) - ord("A") + 27


def part1(input: str) -> int:
    rucksacks = input.splitlines()
    priorities = 0

    for rucksack in rucksacks:
        for letter in rucksack:
            if letter in rucksack[:int(len(rucksack) / 2)] and letter in rucksack[int(len(rucksack) / 2):]:
                priorities += get_priority(letter)
                break

    return priorities


def part2(input: str) -> int:
    rucksacks = input.splitlines()
    priorities = 0

    for i in range(0, len(rucksacks), 3):
        for letter in rucksacks[i]:
            if letter in rucksacks[i + 1] and letter in rucksacks[i + 2]:
                priorities += get_priority(letter)
                break

    return priorities


TEST_INPUT = """vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"""

PART1_TESTS = [
    (TEST_INPUT, 157)
]

PART2_TESTS = [
    (TEST_INPUT, 70)
]
