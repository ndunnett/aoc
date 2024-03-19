from __future__ import annotations
from functools import cache
from enum import Enum, auto


class Condition(Enum):
    Operational = auto()
    Damaged = auto()
    Unknown = auto()

    def parse(ch: str) -> Condition | None:
        match ch:
            case ".": return Condition.Operational
            case "#": return Condition.Damaged
            case "?": return Condition.Unknown
            case _: return None


class Record(tuple):
    @cache
    def __new__(cls, *args) -> Record:
        return tuple.__new__(cls, args)

    @property
    def conds(self) -> tuple[Condition]:
        return self[0]

    @property
    def groups(self) -> tuple[int]:
        return self[1]

    def parse(line: str) -> Record:
        s = line.split()

        return Record(
            tuple(map(Condition.parse, s[0])),
            tuple(map(int, s[1].split(","))),
        )

    def unfolded(self, folds: int) -> Record:
        c, g = self.conds, self.groups

        for _ in range(folds - 1):
            c += (Condition.Unknown,) + self.conds
            g += self.groups

        return Record(c, g)

    @cache
    def solve(self) -> int:
        if not self.conds and not self.groups:
            return 1

        perms = 0

        if self.groups and (g := self.groups[0]) and g <= len(self.conds):
            undetermined = all(c == Condition.Damaged or c == Condition.Unknown for c in self.conds[:g])
            not_damaged = (self.conds[g] != Condition.Damaged) if g < len(self.conds) else True

            if undetermined and not_damaged:
                if g < len(self.conds) and self.conds[g] == Condition.Unknown:
                    perms += Record((Condition.Operational,) + self.conds[g + 1:], self.groups[1:]).solve()
                else:
                    perms += Record(self.conds[g:], self.groups[1:]).solve()

        if self.conds and self.conds[0] != Condition.Damaged:
            perms += Record(self.conds[1:], self.groups).solve()

        return perms


def part1(input: str) -> int:
    return sum(Record.parse(line).solve() for line in input.splitlines())


def part2(input: str) -> int:
    return sum(Record.parse(line).unfolded(5).solve() for line in input.splitlines())


TEST_INPUT = """???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"""

PART1_TESTS = [
    (TEST_INPUT, 21),
]

PART2_TESTS = [
    (TEST_INPUT, 525152),
]
