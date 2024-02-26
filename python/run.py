from importlib import import_module
from urllib.request import Request, urlopen
from pathlib import Path
from os import environ as env
from time import time
from typing import Any, Callable
from argparse import ArgumentParser


CACHE_PATH = Path(__file__).resolve().parent.parent / ".cache"
AOC_SESSION = env.get("AOC_SESSION")

TIME_UNITS = {
    0: "s",
    1: "ms",
    2: "us",
    3: "ns"
}


def download_input(year: int, day: int) -> str:
    """Download puzzle input using AOC_SESSION environment variable"""
    url = f"https://adventofcode.com/{year}/day/{day}/input"
    headers = {"Cookie": f"session={AOC_SESSION}"}

    try:
        with urlopen(Request(url=url, headers=headers)) as response:
            return response.read().decode("utf-8").strip()
    except Exception as e:
        print(f"Failed to download input: {e}")
        exit(1)


def load_input(year: int, day: int) -> str:
    """Load puzzle input from cache or download it"""
    file_path = CACHE_PATH / f"input-{year}-{day:02}.txt"
    CACHE_PATH.resolve().mkdir(parents=True, exist_ok=True)

    if not file_path.is_file():
        downloaded = download_input(year, day)

        with open(file_path, encoding="utf-8", mode="w") as file:
            file.write(downloaded)
            return downloaded

    else:
        with open(file_path, encoding="utf-8", mode="r") as file:
            return str(file.read())


def format_time(time: float) -> str:
    """Format time into a readable string with units"""
    unit = 0

    while time < 1 and unit < 3:
        time *= 1000
        unit += 1

    return f"{round(time, 3)} {TIME_UNITS[unit]}"


def solve(solver: Callable, input: str) -> tuple[Any, float]:
    """Call solver and return result and formatted execution time"""
    start = time()
    return (solver(input), format_time(time() - start))


def run_tests(solver: Callable, tests: list[tuple[str, Any]], part: int) -> None:
    """Call solver with given tests and print results"""
    for n, (input, expected) in enumerate(tests):
        answer, ex_time = solve(solver, input)

        if expected is None:
            result = f"{answer} -- SUCCESS"
        elif answer == expected:
            result = f"{answer} == {expected} -- SUCCESS"
        else:
            result = f"{answer} != {expected} -- FAILURE"

        print(f"Part {part} test {n + 1}: {result} ({ex_time})")


def run_puzzle(solver: Callable, input: str, part: int) -> None:
    """Call solver with given input and print results"""
    answer, ex_time = solve(solver, input)
    print(f"Part {part} answer: {answer} ({ex_time})")


if __name__ == "__main__":
    parser = ArgumentParser(description="Advent of Code runner")
    parser.add_argument("--year", type=int, help="year of the puzzle", required=True)
    parser.add_argument("--day", type=int, help="day of the puzzle", required=True)
    parser.add_argument("--part", type=int, help="part of the puzzle")
    parser.add_argument("--test", action="store_true", help="run tests")
    args = parser.parse_args()
    pkg = import_module(f"{args.year}.day{args.day:02}")

    if args.test:
        if args.part != 2 and hasattr(pkg, "PART1_TESTS"):
            run_tests(pkg.part1, pkg.PART1_TESTS, 1)

        if args.part != 1 and hasattr(pkg, "PART2_TESTS"):
            run_tests(pkg.part2, pkg.PART2_TESTS, 2)

    else:
        input = load_input(args.year, args.day)

        if args.part != 2 and hasattr(pkg, "part1"):
            run_puzzle(pkg.part1, input, 1)

        if args.part != 1 and hasattr(pkg, "part2"):
            run_puzzle(pkg.part2, input, 2)
