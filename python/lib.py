from __future__ import annotations
from functools import cache
from typing import Any, Callable, Generator
from collections.abc import Iterable


def filter_map(func: Callable, it: Iterable) -> Generator[Any, None, None]:
    """Maps function over iterator and yields results that are not None."""
    for element in it:
        if (result := func(element)) is not None:
            yield result


def get_limits(it: Iterable) -> tuple[Any, Any]:
    """Get the lower and upper limits of an iterable."""
    return ((x := sorted(it))[0], x[-1])


def is_intable(s: str) -> bool:
    """Return true if string input can be made an int."""
    try:
        int(s)
        return True
    except:
        return False


def is_floatable(s: str) -> bool:
    """Return true if string input can be made a float."""
    try:
        float(s)
        return True
    except:
        return False


class Point(tuple):
    """Subclass of tuple to support 2 or 3 dimensional points."""

    @cache
    def __new__(cls, *args) -> Point:
        return tuple.__new__(cls, args)

    @property
    def x(self) -> int:
        return self[0]

    @property
    def y(self) -> int:
        return self[1]

    @property
    def z(self) -> int:
        return self[2]

    @cache
    def __add__(l, r: Point) -> Point:
        if len(l) == 2:
            return Point(l.x + r.x, l.y + r.y)
        else:
            return Point(l.x + r.x, l.y + r.y, l.z + r.z)

    @cache
    def __sub__(l, r: Point) -> Point:
        if len(l) == 2:
            return Point(l.x - r.x, l.y - r.y)
        else:
            return Point(l.x - r.x, l.y - r.y, l.z - r.z)

    def __lt__(l, r: Point) -> bool:
        if len(l) == 2:
            return l.x < r.x and l.y < r.y
        else:
            return l.x < r.x and l.y < r.y and l.z < r.z

    def __le__(l, r: Point) -> bool:
        if len(l) == 2:
            return l.x <= r.x and l.y <= r.y
        else:
            return l.x <= r.x and l.y <= r.y and l.z <= r.z

    def __gt__(l, r: Point) -> bool:
        if len(l) == 2:
            return l.x > r.x and l.y > r.y
        else:
            return l.x > r.x and l.y > r.y and l.z > r.z

    def __ge__(l, r: Point) -> bool:
        if len(l) == 2:
            return l.x >= r.x and l.y >= r.y
        else:
            return l.x >= r.x and l.y >= r.y and l.z >= r.z
