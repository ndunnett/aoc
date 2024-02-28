from importlib import import_module
from urllib.request import Request, urlopen
from pathlib import Path
from os import environ
from time import time, perf_counter_ns
from argparse import ArgumentParser, Namespace
from webbrowser import open_new_tab
from typing import Any, Callable


SOURCE_PATH = Path(__file__).resolve().parent
CACHE_PATH = SOURCE_PATH.parent / ".cache"
AOC_SESSION = environ.get("AOC_SESSION")
AOC_URL = "https://adventofcode.com"

TIME_UNITS = {
    0: "ns",
    1: "us",
    2: "ms",
    3: "s",
}

CLI_DESCRIPTION = "Advent of Code puzzle runner. If part is not provided, all parts found will be selected. " \
    "Expected file structure relative to the path of run.py: ./{YEAR}/day{DAY:02}.py"

CLI_EPILOG = "Provide only one action argument. If no action is provided, run will be selected by default."


class ArgParser(ArgumentParser):
    def error(self, message):
        # override error function as a hack to get around unexpected behaviour
        # from exit_on_error=False in order to print help on error
        # https://bugs.python.org/issue41255
        self.exit(2, f"{self.prog}: error: {message}\n\n{self.format_help()}")


def exit_error(message: str, status: int = 1) -> None:
    """Print message out and exit with status code."""
    print(message)
    exit(status)


def download_input(year: int, day: int) -> str:
    """Download puzzle input using AOC_SESSION environment variable."""
    if not AOC_SESSION:
        exit_error("Advent of Code session not found. "
                   "Please define your Advent of Code session as environment variable \"AOC_SESSION\".\n"
                   "\n"
                   "For guidance on finding your session code:\n"
                   "https://github.com/wimglenn/advent-of-code-wim/issues/1")

    url = f"{AOC_URL}/{year}/day/{day}/input"
    headers = {"Cookie": f"session={AOC_SESSION}"}

    try:
        with urlopen(Request(url=url, headers=headers)) as response:
            return response.read().decode("utf-8").strip()
    except Exception as e:
        exit_error(f"Failed to download input: {e}")


def load_input(year: int, day: int) -> str:
    """Load puzzle input from cache or download it."""
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


def format_perf_count(perf_count: int) -> str:
    """Format result from perf_counter_ns into a readable string with units."""
    time = float(perf_count)
    unit = 0

    while time > 1000 and unit < 3:
        time /= 1000
        unit += 1

    return f"{round(time, 3)} {TIME_UNITS[unit]}"


def solve(solver: Callable, input: str, fast: bool = False) -> tuple[Any, str]:
    """Call solver repeatedly for a minimum duration, return the result and formatted median execution time."""
    def _call() -> tuple[Any, int]:
        s = perf_counter_ns()
        return (solver(input), perf_counter_ns() - s)

    start = time()
    result, first_run = _call()
    perf_counts = [first_run]

    if not fast:
        while time() - start < 1:
            res, perf_count = _call()

            if res != result:
                raise Exception(f"Inconsistent solver results: {result} != {res}")

            perf_counts.append(perf_count)

    return (result, format_perf_count(sorted(perf_counts)[len(perf_counts) // 2]))


def parse_args(input: list[str] | None) -> Namespace:
    """Parse and return arguments."""
    parser = ArgParser(description=CLI_DESCRIPTION, epilog=CLI_EPILOG)
    actions = parser.add_argument_group("actions").add_mutually_exclusive_group()
    parser.add_argument("-y", "--year", type=int, help="year of the puzzle to select", required=True)
    parser.add_argument("-d", "--day", type=int, help="day of the puzzle to select", required=True)
    parser.add_argument("-p", "--part", type=int, help="part of the puzzle to select")
    actions.add_argument("-r", "--run", action="store_true", help="run solvers for the puzzle on your input")
    actions.add_argument("-t", "--test", action="store_true", help="run solvers for the puzzle on test inputs")
    actions.add_argument("-i", "--input", action="store_true", help="print the puzzle input")
    actions.add_argument("-o", "--open", action="store_true", help="open the puzzle page in the browser")
    parser.add_argument("-f", "--fast", action="store_true", help="run each solver only once")
    parser._actions = parser._actions[1:] + parser._actions[0:1]  # hack to move [-h] to back of usage
    args = parser.parse_args(input)

    if args.part and args.part not in [1, 2]:
        exit_error(f"Invalid part number: {args.part}\nAdvent of Code puzzles only ever have 1 or 2 parts.")

    return args


def cli(input: list[str] | None = None) -> None:
    """Execute an action on a selected puzzle based on arguments."""
    args = parse_args(input)

    # --open: open the puzzle page in the browser
    if args.open:
        open_new_tab(f"{AOC_URL}/{args.year}/day/{args.day}")

    # --input: print the puzzle input
    elif args.input:
        print(load_input(args.year, args.day))

    else:
        if not (year_path := SOURCE_PATH / str(args.year)).is_dir():
            exit_error(f"Directory not found: {year_path}")

        if not (module_path := year_path / f"day{args.day:02}.py").is_file():
            exit_error(f"Module not found: {module_path}")

        # import the .py file containing the puzzle solution as a module
        module = import_module(f"{args.year}.day{args.day:02}")
        parts = [args.part] if args.part else [1, 2]

        # --test: run solvers for the puzzle on test inputs
        if args.test:
            parts_found = [
                (i, getattr(module, f"part{i}"), getattr(module, f"PART{i}_TESTS")) for i in parts
                if hasattr(module, f"part{i}") and hasattr(module, f"PART{i}_TESTS")
            ]

            for part, solver, tests in parts_found:
                for n, (input, expected) in enumerate(tests):
                    answer, ex_time = solve(solver, input, args.fast)

                    if expected is None:
                        result = f"{answer} -- SUCCESS"
                    elif answer == expected:
                        result = f"{answer} == {expected} -- SUCCESS"
                    else:
                        result = f"{answer} != {expected} -- FAILURE"

                    print(f"Part {part} test {n + 1}: {result} ({ex_time})")

            if len(parts_found) == 0:
                exit_error("No tests found. Tests must be defined in the puzzle module as "
                           "\"PART1_TESTS\" or \"PART2_TESTS\" as a list of tuples with the test "
                           "input as the first index, and the expected result as the second index.")

        # --run: run solvers for the puzzle on your input
        else:
            input = load_input(args.year, args.day)

            parts_found = [
                (i, getattr(module, f"part{i}")) for i in parts
                if hasattr(module, f"part{i}")
            ]

            for part, solver in parts_found:
                answer, ex_time = solve(solver, input, args.fast)
                print(f"Part {part} answer: {answer} ({ex_time})")

            if len(parts_found) == 0:
                exit_error("No solvers found. Solvers must be a callable defined in "
                           "the module scope as \"part1\" or \"part2\".")


if __name__ == "__main__":
    cli()
