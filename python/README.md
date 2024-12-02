# Advent of Code: Python

<details open>
<summary>2024 Solutions</summary>

*Not yet started, I'm working on [Rust](/rust) first.*

</details>

<details>
<summary>2023 Solutions</summary>

* [Day 1: Trebuchet?!](/python/2023/day01.py)
* [Day 2: Cube Conundrum](/python/2023/day02.py)
* [Day 3: Gear Ratios](/python/2023/day03.py)
* [Day 4: Scratchcards](/python/2023/day04.py)
* [Day 5: If You Give A Seed A Fertilizer](/python/2023/day05.py)
* [Day 6: Wait For It](/python/2023/day06.py)
* [Day 7: Camel Cards](/python/2023/day07.py)
* [Day 8: Haunted Wasteland](/python/2023/day08.py)
* [Day 9: Mirage Maintenance](/python/2023/day09.py)
* [Day 10: Pipe Maze](/python/2023/day10.py)
* [Day 11: Cosmic Expansion](/python/2023/day11.py)
* [Day 12: Hot Springs](/python/2023/day12.py)
* [Day 13: Point of Incidence](/python/2023/day13.py)
* [Day 14: Parabolic Reflector Dish](/python/2023/day14.py)
* [Day 15: Lens Library](/python/2023/day15.py)
* [Day 16: The Floor Will Be Lava](/python/2023/day16.py)
* [Day 17: Clumsy Crucible](/python/2023/day17.py)
* [Day 18: Lavaduct Lagoon](/python/2023/day18.py)
* [Day 19: Aplenty](/python/2023/day19.py)
* [Day 20: Pulse Propagation](/python/2023/day20.py)
* [Day 21: Step Counter](/python/2023/day21.py)
* [Day 22: Sand Slabs](/python/2023/day22.py)
* [Day 23: A Long Walk](/python/2023/day23.py)
* [Day 24: Never Tell Me The Odds](/python/2023/day24.py)
* [Day 25: Snowverload](/python/2023/day25.py)

</details>

<details>
<summary>2022 Solutions</summary>

* [Day 1: Calorie Counting](/python/2022/day01.py)
* [Day 2: Rock Paper Scissors](/python/2022/day02.py)
* [Day 3: Rucksack Reorganization](/python/2022/day03.py)
* [Day 4: Camp Cleanup](/python/2022/day04.py)
* [Day 5: Supply Stacks](/python/2022/day05.py)
* [Day 6: Tuning Trouble](/python/2022/day06.py)
* [Day 7: No Space Left On Device](/python/2022/day07.py)
* [Day 8: Treetop Tree House](/python/2022/day08.py)
* [Day 9: Rope Bridge](/python/2022/day09.py)
* [Day 10: Cathode-Ray Tube](/python/2022/day10.py)
* [Day 11: Monkey in the Middle](/python/2022/day11.py)
* [Day 12: Hill Climbing Algorithm](/python/2022/day12.py)
* [Day 13: Distress Signal](/python/2022/day13.py)
* [Day 14: Regolith Reservoir](/python/2022/day14.py)
* [Day 15: Beacon Exclusion Zone](/python/2022/day15.py)
* [Day 16: Proboscidea Volcanium](/python/2022/day16.py)
* [Day 17: Pyroclastic Flow](/python/2022/day17.py)
* [Day 18: Boiling Boulders](/python/2022/day18.py)
* [Day 19: Not Enough Minerals](/python/2022/day19.py)
* [Day 20: Grove Positioning System](/python/2022/day20.py)
* [Day 21: Monkey Math](/python/2022/day21.py)
* [Day 22: Monkey Map](/python/2022/day22.py)
* [Day 23: Unstable Diffusion](/python/2022/day23.py)
* [Day 24: Blizzard Basin](/python/2022/day24.py)
* [Day 25: Full of Hot Air](/python/2022/day25.py)

</details>

## Runner

I've written a basic runner ([run.py](/python/run.py)) which allows you to conveniently run puzzles from a CLI. If you are using my devcontainer, the runner can also be run using the `aoc` command rather than calling the Python script directly. See the help text below for usage:

```text
usage: run.py -y YEAR -d DAY [-p PART] [-r | -t | -i | -o | -s] [-f] [-h]
```

### CLI Arguments

* `-y` / `--year` and `-d` / `--day`: These are required arguments that determine what `.py` file is imported as a module. All other arguments are optional.

* `-p` / `--part`: Allows you select only a specific part to run. By default, both parts will run if found.

* `-r` / `--run`: This is the default option when no other option is selected. This will download your puzzle input and pass it to your solver (imported from your solution as `part1` or `part2` taking a `str` input and returning `Any`) to be run.

* `-t` / `--test`: Imports constants defined in your solution named `PART1_TESTS` and `PART2_TESTS` if found, each being a `list[tuple[str, Any]]`, with the tuples being the test inputs and expected outputs. These will be passed to your solver to be run.

* `-i` / `--input`: Downloads your puzzle input and prints it.

* `-o` / `--open`: Opens the puzzle URL in your browser.

* `-s` / `--start`: Starts a new puzzle by copying `template.py`, then opening the new file (if run inside VSCode), opening the puzzle page in the browser, and printing the puzzle input.

* `-f` / `--fast`: By default, when the solver is run it will repeatedly run for 1 second and print the median execution time. With the `-f` option, it will only run once.

> [!NOTE]
> The runner expects the environment variable `AOC_SESSION` to be set in order to download your input. See wimglenn/advent-of-code-wim#1 for more information.
