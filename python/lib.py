from __future__ import annotations

from typing import TYPE_CHECKING, Callable, Protocol, Self, TypeVar

if TYPE_CHECKING:
    from collections.abc import Generator, Iterable, Iterator, Sequence


T = TypeVar("T", infer_variance=True)
T_contra = TypeVar("T_contra", contravariant=True)
T_co = TypeVar("T_co", covariant=True)


class SupportsMaths(Protocol[T_contra, T_co]):
    def __add__(self, x: T_contra, /) -> T_co: ...
    def __sub__(self, x: T_contra, /) -> T_co: ...
    def __mul__(self, x: T_contra, /) -> T_co: ...


class SupportsRichComparison(Protocol[T]):
    def __lt__(self, other: T, /) -> bool: ...
    def __gt__(self, other: T, /) -> bool: ...


N = TypeVar("N", bound=SupportsMaths)
S = TypeVar("S", bound=SupportsRichComparison)


def filter_map[X, Y](func: Callable[[X], Y | None], it: Iterable[X]) -> Generator[Y, None, None]:
    """Maps function over iterator and yields results that are not None."""
    for element in it:
        if (result := func(element)) is not None:
            yield result


def get_limits(it: Sequence[S]) -> tuple[S, S]:
    """Get the lower and upper limits of an iterable."""
    return ((x := sorted(it))[0], x[-1])


def is_intable(s: str) -> bool:
    """Return true if string input can be made an int."""
    try:
        int(s)
        return True
    except Exception as _:
        return False


def is_floatable(s: str) -> bool:
    """Return true if string input can be made a float."""
    try:
        float(s)
        return True
    except Exception as _:
        return False


class peekable[T]:  # noqa: N801
    it: Iterator[T]
    next_value: T
    next_valid: bool
    end: bool

    def __init__(self, iterable: Iterable[T]) -> None:
        self.it = iter(iterable)

        try:
            self.next_value = next(self.it)
            self.next_valid = True
            self.end = False
        except StopIteration:
            self.next_valid = False
            self.end = True

    def __iter__(self) -> Self:
        return self

    def __next__(self) -> T:
        if self.end:
            raise StopIteration

        if self.next_valid:
            self.next_valid = False
            return self.next_value
        else:
            try:
                return next(self.it)
            except StopIteration as e:
                self.end = True
                raise e

    def __peek__(self) -> T | None:
        if not self.next_valid and not self.end:
            try:
                self.next_value = next(self.it)
                self.next_valid = True
            except StopIteration:
                self.end = True

        if self.end:
            return None
        else:
            return self.next_value


def peek(peekable: peekable[T]) -> T | None:
    return peekable.__peek__()


class chunk_by[T, U]:  # noqa: N801
    it: peekable[T]
    predicate: Callable[[T], U]

    def __init__(self, predicate: Callable[[T], U], iterable: Iterable[T]) -> None:
        self.it = peekable(iterable)
        self.predicate = predicate

    def __iter__(self) -> Self:
        return self

    def __next__(self) -> tuple[U, list[T]]:
        first = next(self.it)
        category = self.predicate(first)
        chunk = [first]

        while (peeked := peek(self.it)) and self.predicate(peeked) == category:
            chunk.append(next(self.it))

        return (category, chunk)


class Point(tuple[N, N]):
    """Subclass of tuple to support 2 dimensional points."""

    __slots__ = ()

    def __new__(cls, *args: N) -> Self:
        return tuple.__new__(cls, args)

    @property
    def x(self) -> N:
        return self[0]

    @property
    def y(self) -> N:
        return self[1]

    def __add__(self, r: Point) -> Point:  # type: ignore
        return Point(self.x + r.x, self.y + r.y)

    def __sub__(self, r: Point) -> Point:
        return Point(self.x - r.x, self.y - r.y)

    def __mul__(self, n: N) -> Point:  # type: ignore
        return Point(self.x * n, self.y * n)

    def __lt__(self, r: Point) -> bool:  # type: ignore
        return self.x < r.x and self.y < r.y

    def __le__(self, r: Point) -> bool:  # type: ignore
        return self.x <= r.x and self.y <= r.y

    def __gt__(self, r: Point) -> bool:  # type: ignore
        return self.x > r.x and self.y > r.y

    def __ge__(self, r: Point) -> bool:  # type: ignore
        return self.x >= r.x and self.y >= r.y


class Point3D(tuple[N, N, N]):
    """Subclass of tuple to support 3 dimensional points."""

    __slots__ = ()

    def __new__(cls, *args: N) -> Self:
        return tuple.__new__(cls, args)

    @property
    def x(self) -> N:
        return self[0]

    @property
    def y(self) -> N:
        return self[1]

    @property
    def z(self) -> N:
        return self[2]

    def __add__(self, r: Point3D) -> Point3D:  # type: ignore
        return Point3D(self.x + r.x, self.y + r.y, self.z + r.z)

    def __sub__(self, r: Point3D) -> Point3D:
        return Point3D(self.x - r.x, self.y - r.y, self.z - r.z)

    def __mul__(self, n: N) -> Point3D:  # type: ignore
        return Point3D(self.x * n, self.y * n, self.z * n)

    def __lt__(self, r: Point3D) -> bool:  # type: ignore
        return self.x < r.x and self.y < r.y and self.z < r.z

    def __le__(self, r: Point3D) -> bool:  # type: ignore
        return self.x <= r.x and self.y <= r.y and self.z <= r.z

    def __gt__(self, r: Point3D) -> bool:  # type: ignore
        return self.x > r.x and self.y > r.y and self.z > r.z

    def __ge__(self, r: Point3D) -> bool:  # type: ignore
        return self.x >= r.x and self.y >= r.y and self.z >= r.z
