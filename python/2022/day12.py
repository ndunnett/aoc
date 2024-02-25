from functools import cache


MAX_DIST = 1_000_000


class Point(tuple):
    @property
    def x(self):
        return self[0]

    @property
    def y(self):
        return self[1]


class Node:
    location: Point
    char: str
    value: int

    def __init__(self, x: int, y: int, char: str):
        self.location = Point((x, y))
        self.char = char

        if char == "E" or char == "S":
            self.value = ord("z") - ord("a")
        else:
            self.value = ord(char) - ord("a")

    @property
    def x(self):
        return self.location.x

    @property
    def y(self):
        return self.location.y


class Map:
    nodes: dict[Point, Node]
    extent: Point

    def __init__(self, input: str):
        lines = input.splitlines()

        self.nodes = {
            Point((x, y)): Node(x, y, char)
            for y, line in enumerate(lines)
            for x, char in enumerate(line)
        }

        self.extent = Point((len(lines[0]) - 1, len(lines) - 1))

    @cache
    def get_neighbours(self, o: Node) -> list[Node]:
        def _gen():
            for i in [Point((o.x, o.y - 1)), Point((o.x, o.y + 1)), Point((o.x - 1, o.y)), Point((o.x + 1, o.y))]:
                if 0 <= i.x <= self.extent.x and 0 <= i.y <= self.extent.y and (n := self.nodes[i]).value - o.value <= 1:
                    yield n
        return list(_gen())

    @cache
    def find_nodes(self, char: str) -> list[Node]:
        def _gen():
            for n in self.nodes.values():
                if n.char == char:
                    yield n
        return list(_gen())

    def bfs(self, start: Node, target_char: str) -> int:
        target = self.find_nodes(target_char)[0]
        visited = set(start.location)
        queue = [(start, 0)]

        while queue:
            node, dist = queue.pop()

            for n in self.get_neighbours(node):
                if n.location not in visited:
                    if n == target:
                        return dist + 1

                    queue = [(n, dist + 1)] + queue
                    visited.add(n.location)

        return MAX_DIST


def part1(input: str) -> int:
    map = Map(input)
    start = map.find_nodes("S")[0]
    return map.bfs(start, "E")


def part2(input: str) -> int:
    map = Map(input)
    paths = [map.bfs(start, "E") for start in map.find_nodes("a")]
    return min(paths)


TEST_INPUT = """Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"""

PART1_TESTS = [
    (TEST_INPUT, 31),
]

PART2_TESTS = [
    (TEST_INPUT, 29),
]
