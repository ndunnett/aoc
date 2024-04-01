from __future__ import annotations
from functools import cache, reduce
from collections import defaultdict


@cache
def hash(s: str) -> int:
    def _hash(acc: int, ch: str) -> int:
        return (acc + ord(ch)) * 17 % 256
    return reduce(_hash, s, 0)


class Lens(tuple):
    def __new__(cls, *args) -> Lens:
        return tuple.__new__(cls, args)

    @property
    def label(self) -> str:
        return self[0]

    @property
    def length(self) -> int | None:
        return self[1]

    @cache
    def parse(chunk: str) -> Lens:
        if chunk.endswith("-"):
            return Lens(chunk[:-1], None)
        else:
            [label, length] = chunk.split("=")
            return Lens(label, int(length))


def part1(input: str) -> int:
    return sum(map(hash, input.strip().split(",")))


def part2(input: str) -> int:
    boxes = defaultdict(list)

    for lens in map(Lens.parse, input.strip().split(",")):
        h = hash(lens.label)

        if lens.length:
            bucket = boxes[h]

            for i, other in enumerate(bucket):
                if other.label == lens.label:
                    bucket[i] = lens
                    break
            else:
                bucket.append(lens)

        elif h in boxes:
            boxes[h] = [other for other in boxes[h] if other.label != lens.label]

    return sum(sum((h + 1) * (i + 1) * lens.length for i, lens in enumerate(box)) for h, box in boxes.items())


TEST_INPUT1 = "HASH"
TEST_INPUT2 = """rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
"""

PART1_TESTS = [
    (TEST_INPUT1, 52),
    (TEST_INPUT2, 1320),
]

PART2_TESTS = [
    (TEST_INPUT2, 145),
]
