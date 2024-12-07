from __future__ import annotations

from dataclasses import dataclass
from functools import reduce
from itertools import chain

from lib import is_intable


@dataclass(frozen=True)
class Expr:
    a: int
    b: int


class Parser:
    PATTERN_DONT = ("o", "n", "'", "t", "(", ")")
    PATTERN_DO = ("o", "(", ")")
    PATTERN_MUL = ("u", "l")

    def __init__(self, input: str) -> None:
        self.chars = tuple(c for c in input)
        self.dont = False
        self.index = 0

    def next(self) -> str | None:
        self.index += 1
        return self.chars[self.index - 1]

    def consume(self, token: str) -> bool:
        if self.chars[self.index] == token:
            self.index += 1
            return True
        else:
            return False

    def match(self, pattern: tuple[str, ...]) -> bool:
        if all(a == b for a, b in zip(self.chars[self.index :], pattern)):
            self.index += len(pattern)
            return True
        else:
            return False

    def parse(self) -> tuple[list[Expr], list[Expr]]:
        do = []
        dont = []

        while self.index < len(self.chars):
            if expr := self.expr():
                if self.dont:
                    dont.append(expr)
                else:
                    do.append(expr)

        return (do, dont)

    def expr(self) -> Expr | None:
        match self.next():
            case "d":
                return self.do()
            case "m":
                return self.mul()
            case _:
                return None

    def do(self) -> Expr | None:
        if self.match(Parser.PATTERN_DONT):
            self.dont = True
        elif self.match(Parser.PATTERN_DO):
            self.dont = False
        return None

    def mul(self) -> Expr | None:
        if (
            self.match(Parser.PATTERN_MUL)
            and self.consume("(")
            and (a := self.number())
            and self.consume(",")
            and (b := self.number())
            and self.consume(")")
        ):
            return Expr(a, b)
        else:
            return None

    def number(self) -> int | None:
        if not is_intable(self.chars[self.index]):
            return None

        end = self.index

        while is_intable(self.chars[end]):
            end += 1

        chars = self.chars[self.index : end]
        self.index = end

        return reduce(
            lambda acc, tup: acc + int(tup[1]) * 10 ** (len(chars) - 1 - tup[0]),
            enumerate(chars),
            0,
        )


class State:
    def __init__(self, input: str) -> None:
        (self.do, self.dont) = Parser(input).parse()

    def mul(self) -> int:
        return reduce(lambda acc, expr: acc + expr.a * expr.b, chain(self.do, self.dont), 0)

    def enabled_mul(self) -> int:
        return reduce(lambda acc, expr: acc + expr.a * expr.b, self.do, 0)


def part1(input: str) -> int:
    return State(input).mul()


def part2(input: str) -> int:
    return State(input).enabled_mul()


TEST_INPUT1 = """xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"""
TEST_INPUT2 = """xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"""

PART1_TESTS = [
    (TEST_INPUT1, 161),
]

PART2_TESTS = [
    (TEST_INPUT2, 48),
]
