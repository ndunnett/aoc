# Advent of Code: Rust

<details open>
<summary>2025 Solutions</summary>

* [Day 1: Secret Entrance](./2025/src/bin/day01.rs)
* [Day 2: Gift Shop](./2025/src/bin/day02.rs)
* [Day 3: Lobby](./2025/src/bin/day03.rs)
* [Day 4: Printing Department](./2025/src/bin/day04.rs)
* [Day 5: Cafeteria](./2025/src/bin/day05.rs)
* [Day 6: Trash Compactor](./2025/src/bin/day06.rs)
* [Day 7: Laboratories](./2025/src/bin/day07.rs)
* [Day 8: Playground](./2025/src/bin/day08.rs)
* [Day 9: Movie Theater](./2025/src/bin/day09.rs)
* [Day 10: Factory](./2025/src/bin/day10.rs)
* [Day 11: Reactor](./2025/src/bin/day11.rs)
* [Day 12: Christmas Tree Farm](./2025/src/bin/day12.rs)

</details>

<details>
<summary>2024 Solutions</summary>

* [Day 1: Historian Hysteria](./2024/src/bin/day01.rs)
* [Day 2: Red-Nosed Reports](./2024/src/bin/day02.rs)
* [Day 3: Mull It Over](./2024/src/bin/day03.rs)
* [Day 4: Ceres Search](./2024/src/bin/day04.rs)
* [Day 5: Print Queue](./2024/src/bin/day05.rs)
* [Day 6: Guard Gallivant](./2024/src/bin/day06.rs)
* [Day 7: Bridge Repair](./2024/src/bin/day07.rs)
* [Day 8: Resonant Collinearity](./2024/src/bin/day08.rs)
* [Day 9: Disk Fragmenter](./2024/src/bin/day09.rs)
* [Day 10: Hoof It](./2024/src/bin/day10.rs)
* [Day 11: Plutonian Pebbles](./2024/src/bin/day11.rs)
* [Day 12: Garden Groups](./2024/src/bin/day12.rs)
* [Day 13: Claw Contraption](./2024/src/bin/day13.rs)
* [Day 14: Restroom Redoubt](./2024/src/bin/day14.rs)
* [Day 15: Warehouse Woes](./2024/src/bin/day15.rs)
* [Day 16: Reindeer Maze](./2024/src/bin/day16.rs)
* [Day 17: Chronospatial Computer](./2024/src/bin/day17.rs)
* [Day 18: RAM Run](./2024/src/bin/day18.rs)
* [Day 19: Linen Layout](./2024/src/bin/day19.rs)
* [Day 20: Race Condition](./2024/src/bin/day20.rs)
* [Day 21: Keypad Conundrum](./2024/src/bin/day21.rs)
* [Day 22: Monkey Market](./2024/src/bin/day22.rs)
* [Day 23: LAN Party](./2024/src/bin/day23.rs)
* [Day 24: Crossed Wires](./2024/src/bin/day24.rs)
* [Day 25: Code Chronicle](./2024/src/bin/day25.rs)

</details>

<details>
<summary>2023 Solutions</summary>

* [Day 1: Trebuchet?!](./2023/src/bin/day01.rs)
* [Day 2: Cube Conundrum](./2023/src/bin/day02.rs)
* [Day 3: Gear Ratios](./2023/src/bin/day03.rs)
* [Day 4: Scratchcards](./2023/src/bin/day04.rs)
* [Day 5: If You Give A Seed A Fertilizer](./2023/src/bin/day05.rs)
* [Day 6: Wait For It](./2023/src/bin/day06.rs)
* [Day 7: Camel Cards](./2023/src/bin/day07.rs)
* [Day 8: Haunted Wasteland](./2023/src/bin/day08.rs)
* [Day 9: Mirage Maintenance](./2023/src/bin/day09.rs)
* [Day 10: Pipe Maze](./2023/src/bin/day10.rs)
* [Day 11: Cosmic Expansion](./2023/src/bin/day11.rs)
* [Day 12: Hot Springs](./2023/src/bin/day12.rs)
* [Day 13: Point of Incidence](./2023/src/bin/day13.rs)
* [Day 14: Parabolic Reflector Dish](./2023/src/bin/day14.rs)
* [Day 15: Lens Library](./2023/src/bin/day15.rs)
* [Day 16: The Floor Will Be Lava](./2023/src/bin/day16.rs)
* [Day 17: Clumsy Crucible](./2023/src/bin/day17.rs)
* [Day 18: Lavaduct Lagoon](./2023/src/bin/day18.rs)
* [Day 19: Aplenty](./2023/src/bin/day19.rs)
* [Day 20: Pulse Propagation](./2023/src/bin/day20.rs)

</details>

<details>
<summary>2022 Solutions</summary>

[/rust/2022/src/bin](./2022/src/bin)

These solutions are merged in from old repos when I was learning Rust, view at your own caution. I plan on rewriting them all.

</details>

## Runner

```text
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

All automated functionality impacting the Advent of Code servers follows the [automation guidelines](https://www.reddit.com/r/adventofcode/wiki/faqs/automation), see [`lib.rs`](./aoc_core/src/lib.rs) in `aoc_core` for implementation details.

* Input files are cached to a local directory and ignored by source control
* The request user agent is set to `github.com/ndunnett/aoc/rust`
* Outbound calls are limited to 5 requests per 15 minutes
