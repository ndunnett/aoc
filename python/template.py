from __future__ import annotations

import math
import operator
import re
from ast import literal_eval
from collections import defaultdict
from copy import deepcopy
from dataclasses import dataclass
from enum import Enum, auto
from functools import cache, reduce
from heapq import heapify, heappop, heappush
from itertools import batched, chain, combinations, cycle, pairwise, permutations, product
from multiprocessing import Pool
from os import cpu_count
from pprint import pprint
from random import random
from statistics import mean, median
from time import time
from typing import Any, Callable, Generator

import numpy as np
import z3

from lib import Point, filter_map, get_limits, is_floatable, is_intable


class State:
    def __init__(self, input: str) -> None:
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
