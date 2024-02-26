from __future__ import annotations
from enum import Enum


class Op(Enum):
    ADD = "+"
    SUB = "-"
    MUL = "*"
    DIV = "/"
    VAR = "variable"
    CONST = "constant"


class Ast:
    a: Ast | None
    b: Ast | None
    n: int | str | None
    op: Op

    def __init__(self, op: Op, a: Ast | None = None, b: Ast | None = None, n: int | str | None = None):
        self.op = op
        self.a = a
        self.b = b
        self.n = n

    def evaluate(self) -> int | str:
        if self.op in [Op.CONST, Op.VAR]:
            return self.n

        match self.op:
            case Op.ADD: return self.a.evaluate() + self.b.evaluate()
            case Op.SUB: return self.a.evaluate() - self.b.evaluate()
            case Op.MUL: return self.a.evaluate() * self.b.evaluate()
            case Op.DIV: return self.a.evaluate() // self.b.evaluate()

    def simplify(self) -> Ast:
        if self.op in [Op.CONST, Op.VAR]:
            return self

        if self.a.op not in [Op.CONST, Op.VAR]:
            self.a = self.a.simplify()

        if self.b.op not in [Op.CONST, Op.VAR]:
            self.b = self.b.simplify()

        if self.a.op == Op.CONST and self.b.op == Op.CONST:
            return Ast(Op.CONST, n=self.evaluate())

        return self


def parse_context(input: str) -> dict[str, str]:
    return {s[0]: s[1].strip() for s in [line.split(":") for line in input.splitlines()]}


def is_number(s: str) -> bool:
    try:
        int(s)
        return True
    except:
        return False


def parse_ast(expr: str, context: dict[str, str]) -> Ast:
    if expr in context:
        return parse_ast(context[expr], context)

    elif is_number(expr):
        return Ast(Op.CONST, n=int(expr))

    elif any(s in expr for s in "+-*/"):
        parts = expr.split()
        return Ast(
            Op._value2member_map_[parts[1]],
            a=parse_ast(parts[0], context),
            b=parse_ast(parts[2], context)
        )

    else:
        return Ast(Op.VAR, n=expr)


def solve(left: Ast, right: Ast) -> int:
    c, v = (left, right) if left.op == Op.CONST else (right, left)

    if v.op == Op.VAR:
        return c.evaluate()

    if v.a.op == Op.CONST:
        match v.op:
            case Op.ADD: c.n -= v.a.evaluate()
            case Op.SUB: c.n = v.a.evaluate() - c.n
            case Op.MUL: c.n //= v.a.evaluate()
            case Op.DIV: c.n = v.a.evaluate() // c.n
        return solve(v.b, c)

    else:
        match v.op:
            case Op.ADD: c.n -= v.b.evaluate()
            case Op.SUB: c.n += v.b.evaluate()
            case Op.MUL: c.n //= v.b.evaluate()
            case Op.DIV: c.n *= v.b.evaluate()
        return solve(v.a, c)


def part1(input: str) -> int:
    m = parse_context(input)
    return parse_ast(m["root"], m).evaluate()


def part2(input: str) -> int:
    m = parse_context(input)
    m["humn"] = "x"
    ast = parse_ast(m["root"], m).simplify()
    return solve(ast.a, ast.b)


TEST_INPUT = """root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32"""

PART1_TESTS = [
    (TEST_INPUT, 152),
]

PART2_TESTS = [
    (TEST_INPUT, 301),
]
