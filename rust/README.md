# Advent of Code: Rust

## Solutions

So far these are only old solutions merged in from old repos.

## Runner

```
Usage: aoc [OPTIONS] --year <YEAR> --day <DAY>

Options:
  -y, --year <YEAR>  Year of the puzzle to select
  -d, --day <DAY>    Day of the puzzle to select
  -p, --part <PART>  Part of the puzzle to select (omit to run all parts)
  -r, --run          Run solution on selected puzzle
  -t, --test         Run tests for selected puzzle
  -i, --input        Print puzzle input
  -n, --new          Start new puzzle from template
  -o, --open         Open puzzle page in browser
      --release      Run in release mode
  -h, --help         Print help
  -V, --version      Print version
```

> [!NOTE]
> The runner expects the environment variable `AOC_SESSION` to be set in order to download your input. See wimglenn/advent-of-code-wim#1 for more information.

### Automation Compliance

All automation impacting Advent of Code servers follows the [automation guidelines](https://www.reddit.com/r/adventofcode/wiki/faqs/automation), see [`input.rs`](/rust/aoc_core/src/input.rs) for implementation.

* Input files are cached to a local directory
* The request user agent is set to `github.com/ndunnett/aoc/rust`
* Outbound calls are limited to 5 requests per 15 minutes
