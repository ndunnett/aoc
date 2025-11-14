use std::mem::MaybeUninit;

#[inline(always)]
const fn gcd(mut a: u64, mut b: u64) -> u64 {
    if a == 0 {
        return b;
    }

    if b == 0 {
        return a;
    }

    let shift = (a | b).trailing_zeros();
    a >>= shift;
    b >>= shift;
    a >>= a.trailing_zeros();

    loop {
        b >>= b.trailing_zeros();

        if a > b {
            std::mem::swap(&mut a, &mut b);
        }

        b -= a;

        if b == 0 {
            break;
        }
    }

    a << shift
}

#[inline(always)]
fn lcm<I: IntoIterator<Item = u64>>(nums: I) -> u64 {
    let mut lcm = 1;

    for n in nums {
        lcm = n * lcm / gcd(n, lcm);
    }

    lcm
}

#[derive(Clone, Copy, PartialEq)]
enum Pulse {
    Low,
    High,
}

type Id = u16;
type IdVec = ArrayVec<Id, 8, u8>;

#[derive(Clone, Copy)]
struct Message {
    source: Id,
    destination: Id,
    pulse: Pulse,
}

impl Message {
    const BUTTON_PRESS: Self = Self {
        source: Id::MAX,
        destination: 0,
        pulse: Pulse::Low,
    };
}

#[derive(Clone)]
enum ModuleFunction {
    Broadcast,
    FlipFlop { state: bool },
    Conjunction { state: FxHashMap<Id, Pulse> },
}

impl ModuleFunction {
    #[inline(always)]
    fn process(&mut self, message: Message) -> Option<Pulse> {
        match self {
            ModuleFunction::Broadcast => Some(message.pulse),
            ModuleFunction::FlipFlop { state } => {
                if message.pulse == Pulse::Low {
                    *state = !*state;
                    Some(if *state { Pulse::High } else { Pulse::Low })
                } else {
                    None
                }
            }
            ModuleFunction::Conjunction { state } => {
                state.insert(message.source, message.pulse);

                if state.values().all(|pulse| pulse == &Pulse::High) {
                    Some(Pulse::Low)
                } else {
                    Some(Pulse::High)
                }
            }
        }
    }
}

#[derive(Clone)]
struct Module {
    id: Id,
    destinations: IdVec,
    function: ModuleFunction,
}

/// Parse an ID from a slice of bytes starting at index 'i'
#[inline(always)]
const fn parse_id(bytes: &[u8], i: &mut usize) -> Id {
    let mut id = 0;

    while *i < bytes.len() && bytes[*i].is_ascii_alphabetic() {
        id = id * 27 + (bytes[*i] - b'a' + 1) as Id;
        *i += 1;
    }

    id
}

const RX_ID: Id = parse_id(b"rx", &mut 0);

#[derive(Clone, Copy)]
struct FiLoQueue<const CAPACITY: usize> {
    data: [MaybeUninit<Message>; CAPACITY],
    len: u8,
    next_pop: u8,
    next_push: u8,
}

impl<const CAPACITY: usize> FiLoQueue<CAPACITY> {
    #[inline(always)]
    const fn new() -> Self {
        Self {
            data: [MaybeUninit::uninit(); CAPACITY],
            len: 0,
            next_pop: 0,
            next_push: 0,
        }
    }

    #[inline(always)]
    fn push(&mut self, value: Message) {
        if self.len as usize >= CAPACITY {
            panic!("queue too small");
        }

        unsafe {
            self.data
                .get_unchecked_mut(self.next_push as usize)
                .write(value)
        };

        self.next_push = (self.next_push + 1) % CAPACITY as u8;
        self.len += 1;
    }

    #[inline(always)]
    fn pop(&mut self) -> Option<Message> {
        if self.len > 0 {
            let value = Some(unsafe {
                self.data
                    .get_unchecked(self.next_pop as usize)
                    .assume_init()
            });
            self.next_pop = (self.next_pop + 1) % CAPACITY as u8;
            self.len -= 1;
            value
        } else {
            None
        }
    }

    #[inline(always)]
    fn clear(&mut self) {
        self.len = 0;
        self.next_pop = 0;
        self.next_push = 0;
    }
}

#[derive(Clone)]
struct Solution {
    machine: FxHashMap<Id, Module>,
}

impl Solver for Solution {
    #[inline(always)]
    fn new(input: &str) -> Anyhow<Self> {
        let bytes = input.as_bytes();
        let mut i = 0;
        let mut machine = FxHashMap::with_capacity_and_hasher(60, FxBuildHasher);

        while i < bytes.len() {
            // Parse module ID and function
            let (id, function) = match bytes[i] {
                b'b' => {
                    // Skip past "broadcaster"
                    i += 11;

                    (0, ModuleFunction::Broadcast)
                }
                b'%' => {
                    i += 1;

                    (
                        parse_id(bytes, &mut i),
                        ModuleFunction::FlipFlop { state: false },
                    )
                }
                b'&' => {
                    i += 1;

                    (
                        parse_id(bytes, &mut i),
                        ModuleFunction::Conjunction {
                            state: FxHashMap::with_hasher(FxBuildHasher),
                        },
                    )
                }
                _ => break,
            };

            // Skip past " -> "
            i += 4;

            // Parse destination IDs
            let mut destinations = IdVec::new();

            while i < bytes.len() {
                destinations.push(parse_id(bytes, &mut i));

                // Skip past ", " otherwise assume new line
                if bytes[i] == b',' {
                    i += 2;
                } else {
                    i += 1;
                    break;
                }
            }

            machine.insert(
                id,
                Module {
                    id,
                    destinations,
                    function,
                },
            );
        }

        // Collect connections of conjunction modules sinking from other modules
        let connections = machine
            .values()
            .flat_map(|module| {
                module.destinations.iter().filter_map(|dest| {
                    if let Some(Module {
                        function: ModuleFunction::Conjunction { .. },
                        ..
                    }) = machine.get(dest)
                    {
                        Some((module.id, *dest))
                    } else {
                        None
                    }
                })
            })
            .collect::<Vec<_>>();

        // Initialise conjunction module state for all possible sources
        for (src, dst) in connections.into_iter() {
            if let Some(Module {
                function: ModuleFunction::Conjunction { state },
                ..
            }) = machine.get_mut(&dst)
            {
                state.insert(src, Pulse::Low);
            }
        }

        Ok(Self { machine })
    }

    fn part1(&mut self) -> Anyhow<impl fmt::Display> {
        let mut queue = FiLoQueue::<64>::new();
        let mut low = 0;
        let mut high = 0;

        for _ in 0..1000 {
            queue.clear();
            queue.push(Message::BUTTON_PRESS);

            while let Some(message) = queue.pop() {
                match message.pulse {
                    Pulse::Low => low += 1,
                    Pulse::High => high += 1,
                }

                if let Some(module) = self.machine.get_mut(&message.destination)
                    && let Some(pulse) = module.function.process(message)
                {
                    for &destination in module.destinations.as_slice() {
                        queue.push(Message {
                            source: module.id,
                            destination,
                            pulse,
                        });
                    }
                }
            }
        }

        Ok(low * high)
    }

    fn part2(&mut self) -> Anyhow<impl fmt::Display> {
        let final_gate = self
            .machine
            .values()
            .find_map(|module| {
                if module.destinations.iter().contains(&RX_ID) {
                    Some(module.id)
                } else {
                    None
                }
            })
            .ok_or_else(|| anyhow!("failed to find 'rx' gate"))?;

        let mut counters = self
            .machine
            .values()
            .filter_map(|module| {
                if module.destinations.iter().contains(&final_gate) {
                    Some((module.id, 0))
                } else {
                    None
                }
            })
            .collect::<FxHashMap<_, _>>();

        let mut queue = FiLoQueue::<64>::new();
        let mut i = 0;

        loop {
            queue.clear();
            queue.push(Message::BUTTON_PRESS);
            i += 1;

            while let Some(message) = queue.pop() {
                if message.pulse == Pulse::High
                    && let Some(count) = counters.get_mut(&message.source)
                {
                    *count = i;

                    if counters.values().all(|&count| count > 0) {
                        return Ok(lcm(counters.values().copied()));
                    }
                }

                if let Some(module) = self.machine.get_mut(&message.destination)
                    && let Some(pulse) = module.function.process(message)
                {
                    for &destination in module.destinations.as_slice() {
                        queue.push(Message {
                            source: module.id,
                            destination,
                            pulse,
                        });
                    }
                }
            }
        }
    }
}

aoc::solution!();

#[cfg(test)]
mod test {
    use super::{Solution, Solver};

    const INPUT_1: &str = r"broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";

    const INPUT_2: &str = r"broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";

    #[test]
    fn test_part1_1() {
        let mut solution = Solution::new(INPUT_1).unwrap();
        let answer = solution.part1().unwrap().to_string();
        assert_eq!(answer, "32000000");
    }

    #[test]
    fn test_part1_2() {
        let mut solution = Solution::new(INPUT_2).unwrap();
        let answer = solution.part1().unwrap().to_string();
        assert_eq!(answer, "11687500");
    }
}
