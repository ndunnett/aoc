# Advent of Code: Rust

## Solutions

### 2024

* [Day 1: Historian Hysteria](/rust/2024/src/bin/day01.rs)

### 2023

* [Day 1: Trebuchet?!](/rust/2023/src/bin/day01.rs)
* [Day 2: Cube Conundrum](/rust/2023/src/bin/day02.rs)
* [Day 3: Gear Ratios](/rust/2023/src/bin/day03.rs)
* [Day 4: Scratchcards](/rust/2023/src/bin/day04.rs)
* [Day 5: If You Give A Seed A Fertilizer](/rust/2023/src/bin/day05.rs)
* [Day 6: Wait For It](/rust/2023/src/bin/day06.rs)
* [Day 7: Camel Cards](/rust/2023/src/bin/day07.rs)

Other solutions are merged in from old repos when I was learning Rust, view at your own caution. I plan on rewriting them all.

## Runner

```
Usage: aoc [OPTIONS] --year <YEAR> --day <DAY>

Options:
  -y, --year <YEAR>  Year of the puzzle to select
  -d, --day <DAY>    Day of the puzzle to select
  -p, --part <PART>  Part of the puzzle to select (omit to run all parts)
  -r, --run          Run the selected puzzle (default if no other options are provided)
      --release      Run in release mode
  -t, --test         Run tests for the selected puzzle
  -i, --input        Print the selected puzzle input
  -n, --new          Start new puzzle from template
      --no-cache     Force redownloading the input and overwrite the cached file
  -o, --open         Open the selected puzzle page in browser
  -h, --help         Print help
  -V, --version      Print version
```

> [!NOTE]
> The runner expects the environment variable `AOC_SESSION` to be set in order to download your input. See wimglenn/advent-of-code-wim#1 for more information.

### Automation Compliance

All automated functionality impacting the Advent of Code servers follows the [automation guidelines](https://www.reddit.com/r/adventofcode/wiki/faqs/automation), see [`lib.rs`](/rust/aoc_core/src/lib.rs) for implementation.

* Input files are cached to a local directory and ignored by source control
* The request user agent is set to `github.com/ndunnett/aoc/rust`
* Outbound calls are limited to 5 requests per 15 minutes
