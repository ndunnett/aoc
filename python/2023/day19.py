from __future__ import annotations
from functools import reduce
from enum import Enum, auto
from dataclasses import dataclass


class Condition(Enum):
    LessThan = auto()
    GreaterThan = auto()
    Accept = auto()
    Reject = auto()

    def evaluate(self, left: int, right: int) -> bool:
        match self:
            case Condition.LessThan: return left < right
            case Condition.GreaterThan: return left > right
            case Condition.Accept: return True
            case Condition.Reject: return False


@dataclass
class Node:
    condition: Condition
    left: int | None
    right: int | None
    branch_true: Node | None
    branch_false: Node | None

    def parse(wfs: dict[str, list[str]], rules: list[str] = ["in"]) -> Node:
        if ":" not in (rule := rules[0]):
            match rule:
                case "A": return Node(Condition.Accept, None, None, None, None)
                case "R": return Node(Condition.Reject, None, None, None, None)
                case _: return Node.parse(wfs, wfs[rule])
        else:
            exp, ev = rule.split(":")
            return Node(
                Condition.GreaterThan if ">" in exp else Condition.LessThan,
                "xmas".index(exp[0]),
                int(exp[2:]),
                Node.parse(wfs, [ev]),
                Node.parse(wfs, rules[1:]),
            )

    def evaluate(self, part: tuple[int]) -> bool:
        match self.condition:
            case Condition.Accept: return True
            case Condition.Reject: return False
            case _:
                if self.condition.evaluate(part[self.left], self.right):
                    return self.branch_true.evaluate(part)
                else:
                    return self.branch_false.evaluate(part)

    def combinations(self, ranges: dict[int, range]) -> int:
        match self.condition:
            case Condition.Accept:
                return reduce(lambda a, b: a * b, (r.stop - r.start + 1 for r in ranges.values()))
            case Condition.Reject:
                return 0
            case Condition.LessThan:
                true_ranges = ranges | {self.left: range(ranges[self.left].start, self.right - 1)}
                false_ranges = ranges | {self.left: range(self.right, ranges[self.left].stop)}
            case Condition.GreaterThan:
                true_ranges = ranges | {self.left: range(self.right + 1, ranges[self.left].stop)}
                false_ranges = ranges | {self.left: range(ranges[self.left].start, self.right)}

        return self.branch_true.combinations(true_ranges) + self.branch_false.combinations(false_ranges)


def parse_workflows(section: str) -> dict[str, list[str]]:
    return {
        s[0]: s[1][:-1].split(",")
        for line in section.splitlines()
        if (s := line.split("{"))
    }


def parse_part(line: str) -> tuple[int]:
    s = line.replace("}", "").split(",")
    return tuple(map(lambda p: int(p.split("=")[-1]), s))


def part1(input: str) -> int:
    sections = input.split("\n\n")
    dt = Node.parse(parse_workflows(sections[0]))
    return sum(sum(part) for line in sections[1].splitlines() if dt.evaluate(part := parse_part(line)))


def part2(input: str) -> int:
    return Node.parse(parse_workflows(input.split("\n\n")[0])).combinations({i: range(1, 4000) for i in range(4)})


TEST_INPUT = """px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}"""

PART1_TESTS = [
    (TEST_INPUT, 19114),
]

PART2_TESTS = [
    (TEST_INPUT, 167409079868000),
]
