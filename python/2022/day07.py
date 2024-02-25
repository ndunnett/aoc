from __future__ import annotations
from functools import cache


DISK_SIZE = 70000000
SPACE_NEEDED = 30000000


class Node:
    name: str
    size: int
    children: dict[str, Node]

    def __init__(self, name: str, size: int):
        self.name = name
        self.size = size
        self.children = {}

    def add_file(self, path: list[str], name: str, size: int) -> None:
        if len(path) == 0:
            self.children[name] = File(name, size)
        else:
            self.children[path[0]].add_file(path[1:], name, size)

    def add_dir(self, path: list[str], name: str) -> None:
        if len(path) == 0:
            self.children[name] = Dir(name)
        else:
            self.children[path[0]].add_dir(path[1:], name)


class File(Node):
    def __init__(self, name: str, size: int):
        super().__init__(name, size)

    @property
    def total_size(self) -> int:
        return self.size


class Dir(Node):
    def __init__(self, name: str):
        super().__init__(name, 0)

    @property
    @cache
    def total_size(self) -> int:
        return sum(child.total_size for child in self.children.values())


class Root(Dir):
    def __init__(self, name: str = "/"):
        super().__init__(name)
        self.add_dir([], name)


def parse(input: str) -> Root:
    sections = filter(lambda s: s, input.split("$"))
    location = []
    root = Root()

    for section in sections:
        lines = section.strip().splitlines()
        tokens = lines[0].split()
        arg = None if len(tokens) == 1 else tokens[1]

        match (tokens[0], arg):
            case ("cd", ".."): location.pop()
            case ("cd", _): location.append(arg)
            case ("ls", _):
                for output in [line.split() for line in lines[1:]]:
                    if output[0] == "dir":
                        root.add_dir(location, output[1])
                    else:
                        root.add_file(location, output[1], int(output[0]))

    return root


def get_dir_sizes(node: Node) -> int:
    dir_sizes = sum(get_dir_sizes(child) for child in node.children.values())

    if issubclass(type(node), Dir) and node.total_size <= 100000:
        dir_sizes += node.total_size

    return dir_sizes


def find_dir_to_delete(node: Node, deficit: int) -> int:
    answer = DISK_SIZE

    for child in node.children.values():
        if child.total_size >= deficit:
            size = find_dir_to_delete(child, deficit)

            if size >= deficit and size < answer:
                answer = size

    if node.total_size >= deficit and node.total_size < answer and issubclass(type(node), Dir):
        answer = node.total_size

    return answer


def part1(input: str) -> int:
    return get_dir_sizes(parse(input))


def part2(input: str) -> int:
    root = parse(input)
    return find_dir_to_delete(root, root.total_size + SPACE_NEEDED - DISK_SIZE)


TEST_INPUT = """$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"""

PART1_TESTS = [
    (TEST_INPUT, 95437),
]

PART2_TESTS = [
    (TEST_INPUT, 24933642),
]
