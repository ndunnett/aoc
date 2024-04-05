from __future__ import annotations
from functools import reduce
from collections import defaultdict
from random import choice


def part1(input: str) -> int:
    graph = defaultdict(list)

    for line in input.splitlines():
        src, dests = line.split(": ")
        dests = dests.split()
        graph[src] += dests

        for dest in dests:
            graph[dest].append(src)

    while True:
        _graph = {k: v[:] for k, v in graph.items()}
        edges = {node: 1 for node in _graph}

        while len(_graph) > 2:
            u = choice(list(_graph.keys()))
            v = choice(_graph[u])
            _graph[u] += _graph[v]

            for node in _graph[v]:
                _graph[node].remove(v)
                _graph[node].append(u)

            _graph[u] = list(filter(lambda node: node != u, _graph[u]))
            _graph.pop(v)
            edges[u] += edges.pop(v)

        if len(_graph[list(_graph.keys())[0]]) == 3:
            return reduce(lambda a, b: a * b, edges.values())


TEST_INPUT = """jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr"""

PART1_TESTS = [
    (TEST_INPUT, 54),
]
