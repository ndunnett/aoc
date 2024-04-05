from __future__ import annotations
from lib import Point, is_intable, is_floatable, get_limits, filter_map
from functools import cache, reduce
from itertools import chain, pairwise, batched, combinations, permutations, product, cycle
from heapq import heapify, heappop, heappush
from enum import Enum, auto
from copy import deepcopy
from collections import defaultdict
from dataclasses import dataclass
from ast import literal_eval
from typing import Any, Callable, Generator
from random import random
from statistics import median, mean
from multiprocessing import Pool
from os import cpu_count
import z3
import math
import re
import operator
from pprint import pprint
from time import time


class State:
    def __init__(self, input: str):
        lines = input.splitlines()
        pass

    def solve(self) -> int:
        pass


def part1(input: str) -> int:
    return State(input).solve()


def part2(input: str) -> int:
    return State(input).solve()


TEST_INPUT = """
"""

PART1_TESTS = [
    (TEST_INPUT, None),
]

PART2_TESTS = [
    (TEST_INPUT, None),
]
