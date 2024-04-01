from __future__ import annotations
from enum import Enum, auto
from collections import defaultdict
from dataclasses import dataclass
from math import lcm


class Pulse(Enum):
    Low = auto()
    High = auto()


class Message(tuple):
    def __new__(cls, *args) -> Message:
        return tuple.__new__(cls, args)

    @property
    def source(self) -> str:
        return self[0]

    @property
    def destination(self) -> str:
        return self[1]

    @property
    def pulse(self) -> Pulse:
        return self[2]


@dataclass
class Module:
    name: str
    destinations: tuple[str]

    def parse(line: str) -> tuple[str, Module]:
        decl, dests = line.split(" -> ")

        match decl[0]:
            case "b": module = Broadcaster(decl, tuple(dests.split(", ")))
            case "%": module = FlipFlop(decl[1:], tuple(dests.split(", ")))
            case "&": module = Conjunction(decl[1:], tuple(dests.split(", ")))

        try:
            module.state = module.initial_state()
        except:
            pass

        return module


class Output(Module):
    def process(self, msg: Message) -> None:
        return None


class Broadcaster(Module):
    def process(self, msg: Message) -> Pulse:
        return msg.pulse


class FlipFlop(Module):
    state: bool

    def initial_state(self) -> bool:
        return False

    def process(self, msg: Message) -> Pulse | None:
        if msg.pulse == Pulse.Low:
            self.state = not self.state
            return Pulse.High if self.state else Pulse.Low
        else:
            return None


class Conjunction(Module):
    state: dict[str, Pulse]

    def initial_state(self) -> dict[str, Pulse]:
        return dict()

    def process(self, msg: Message) -> Pulse:
        self.state[msg.source] = msg.pulse
        if all(s == Pulse.High for s in self.state.values()):
            return Pulse.Low
        else:
            return Pulse.High


def parse_modules(input: str) -> dict[str, Module]:
    modules = defaultdict(lambda: Output("output", tuple()), {
        (module := Module.parse(line)).name: module
        for line in input.splitlines()
    })

    for name, module in modules.items():
        for dest in module.destinations:
            if dest in modules and isinstance(modules[dest], Conjunction):
                modules[dest].state[name] = Pulse.Low

    return modules


BUTTON_PRESS = Message("button", "broadcaster", Pulse.Low)


def part1(input: str) -> int:
    modules = parse_modules(input)
    low, high = 0, 0

    for _ in range(1000):
        queue = [BUTTON_PRESS]

        while queue and (msg := queue.pop(0)):
            match msg.pulse:
                case Pulse.Low: low += 1
                case Pulse.High: high += 1

            module = modules[msg.destination]

            if (out := module.process(msg)):
                for dest in module.destinations:
                    queue.append(Message(module.name, dest, out))

    return low * high


def part2(input: str) -> int:
    modules = parse_modules(input)
    final_gate = next(module.name for module in modules.values() if "rx" in module.destinations)
    counters = {module.name: 0 for module in modules.values() if final_gate in module.destinations}
    i = 0

    while (i := i + 1):
        queue = [BUTTON_PRESS]

        while queue and (msg := queue.pop(0)):
            if msg.pulse == Pulse.High and msg.source in counters:
                counters[msg.source] = i

                if all(counters.values()):
                    return lcm(*counters.values())

            module = modules[msg.destination]

            if (out := module.process(msg)):
                for dest in module.destinations:
                    queue.append(Message(module.name, dest, out))


TEST_INPUT1 = r"""broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a"""

TEST_INPUT2 = r"""broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output"""

PART1_TESTS = [
    (TEST_INPUT1, 32000000),
    (TEST_INPUT2, 11687500),
]
