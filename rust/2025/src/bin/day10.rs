#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use std::mem::MaybeUninit;

use z3::{SatResult, ast::Int};

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct LightSet {
    data: u16,
}

impl LightSet {
    #[inline(always)]
    const fn new() -> Self {
        Self { data: 0 }
    }

    #[inline(always)]
    const fn insert(&mut self, index: usize) {
        self.data |= 1 << index;
    }

    #[inline(always)]
    const fn difference(self, other: Self) -> Self {
        Self {
            data: self.data ^ other.data,
        }
    }

    #[inline(always)]
    const fn as_usize(&self) -> usize {
        self.data as usize
    }

    #[inline(always)]
    fn iter(&self) -> impl Iterator<Item = usize> {
        let mut data = self.data;

        std::iter::from_fn(move || {
            if data == 0 {
                None
            } else {
                let next = data.trailing_zeros() as usize;
                data &= data - 1;
                Some(next)
            }
        })
    }
}

type ButtonsVec = MicroVec<LightSet, 16, u8>;
type JoltagesVec = MicroVec<u16, 10, u8>;

#[derive(Clone)]
struct Machine {
    lights: LightSet,
    buttons: ButtonsVec,
    joltages: JoltagesVec,
}

impl Machine {
    fn parser(input: &str) -> impl Iterator<Item = Self> {
        let bytes = input.as_bytes();
        let ptr_range = bytes.as_ptr_range();
        let p_end = ptr_range.end;
        let mut p = ptr_range.start;

        std::iter::from_fn(move || {
            // Safety: beware, raw pointer arithmatic inside
            unsafe {
                // Ensure the line is not empty
                if p >= p_end {
                    return None;
                }

                // Skip '['
                p = p.add(1);

                // Parse lights
                let mut len = 0;
                let mut lights = LightSet::new();

                loop {
                    match *p.add(len) {
                        // Reached end of the lights
                        b']' => {
                            p = p.add(len + 1);
                            break;
                        }
                        // Reached a true state
                        b'#' => {
                            lights.insert(len);
                        }
                        _ => {}
                    }

                    len += 1;
                }

                // Parse buttons
                let mut buttons = ButtonsVec::new();

                'outer: loop {
                    let mut button = LightSet::new();

                    'inner: loop {
                        match *p {
                            // Reached the end of the button
                            b')' => {
                                // Skip ') '
                                p = p.add(2);
                                buttons.push(button);
                                break 'inner;
                            }
                            // Reached the beginning of joltages
                            b'{' => {
                                break 'outer;
                            }
                            // Reached a button element
                            b if b.is_ascii_digit() => {
                                button.insert((b - b'0') as usize);
                            }
                            _ => {}
                        }

                        p = p.add(1);
                    }
                }

                // Skip to the next digit
                while !(*p).is_ascii_digit() {
                    p = p.add(1);
                }

                // Parse joltages
                let mut joltages = JoltagesVec::new();

                'outer: loop {
                    let mut joltage = 0;

                    'inner: loop {
                        match *p {
                            // Reached the end of the current joltage
                            b',' => {
                                // Skip ','
                                p = p.add(1);
                                joltages.push(joltage);
                                break 'inner;
                            }
                            // Reached the end of joltages
                            b'}' => {
                                // Skip '}\n'
                                p = p.add(2);
                                joltages.push(joltage);
                                break 'outer;
                            }
                            // Reached a digit to parse
                            b if b.is_ascii_digit() => {
                                joltage = joltage * 10 + (b - b'0') as u16;
                            }
                            _ => {}
                        }

                        p = p.add(1);
                    }
                }

                Some(Machine {
                    lights,
                    buttons,
                    joltages,
                })
            }
        })
    }
}

/// Asserts that `N` is a power of 2 at compile time
struct IsPowerOfTwo<const N: usize>;
impl<const N: usize> utility::True for IsPowerOfTwo<N> where
    utility::Guard<{ N.is_power_of_two() }>: utility::True
{
}

/// First-in-last-out minimal queue, requires `CAPACITY` to be a power of 2
struct FiLoQueue<T, const CAPACITY: usize>
where
    IsPowerOfTwo<CAPACITY>: utility::True,
{
    data: [MaybeUninit<T>; CAPACITY],
    head: usize,
    tail: usize,
}

impl<T: Copy, const CAPACITY: usize> FiLoQueue<T, CAPACITY>
where
    IsPowerOfTwo<CAPACITY>: utility::True,
{
    const MASK: usize = CAPACITY - 1;

    #[inline(always)]
    const fn new() -> Self {
        Self {
            data: [MaybeUninit::uninit(); CAPACITY],
            head: 0,
            tail: 0,
        }
    }

    /// Safety:
    /// No bounds checks, make sure `CAPACITY` is big enough
    #[inline(always)]
    const unsafe fn push_unchecked(&mut self, value: T) {
        unsafe { (*self.data.as_mut_ptr().add(self.tail)).write(value) };
        self.tail = (self.tail + 1) & Self::MASK;
    }

    #[inline(always)]
    const fn pop(&mut self) -> Option<T> {
        if self.head == self.tail {
            return None;
        }

        // Safety: `self.head` will always be within bounds and initialised
        let value = unsafe { (*self.data.as_ptr().add(self.head)).assume_init() };
        self.head = (self.head + 1) & Self::MASK;
        Some(value)
    }
}

#[derive(Clone)]
struct Solution {
    machines: Vec<Machine>,
}

impl Solver for Solution {
    fn new(input: &str) -> Anyhow<Self> {
        Ok(Self {
            machines: Machine::parser(input).collect(),
        })
    }

    fn part1(&mut self) -> Anyhow<impl fmt::Display> {
        const NEW_CACHE: [u16; 1 << 10] = {
            let mut cache = [u16::MAX; _];
            cache[0] = 0;
            cache
        };

        Ok(self
            .machines
            .iter()
            .map(|machine| {
                let mut queue = FiLoQueue::<LightSet, 1024>::new();
                unsafe { queue.push_unchecked(LightSet::new()) };
                let mut cache = NEW_CACHE;

                // Perform BFS to find the optimal button combination
                while let Some(state) = queue.pop() {
                    let presses = cache[state.as_usize()] + 1;

                    for &button in &machine.buttons {
                        // Pressing a button equates to a set difference between the current state and the button
                        let next_state = state.difference(button);

                        if next_state == machine.lights {
                            return presses;
                        }

                        if cache[next_state.as_usize()] > presses {
                            cache[next_state.as_usize()] = presses;
                            // Safety: `queue` should be large enough for the search space
                            unsafe { queue.push_unchecked(next_state) };
                        }
                    }
                }

                // Safety: the input is friendly and always has a solution
                unsafe { std::hint::unreachable_unchecked() }
            })
            .sum::<u16>())
    }

    fn part2(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self
            .machines
            .par_iter()
            .map(|machine| {
                let solver = z3::Solver::new();

                // Make a term for each joltage
                let joltage_terms = (0..machine.buttons.len())
                    .map(|_| Int::fresh_const("x"))
                    .collect::<MicroVec<_, 16, u8>>();

                // Constrain the joltage terms to be non-negative
                for term in &joltage_terms {
                    solver.assert(term.ge(Int::from_u64(0)));
                }

                // Constrain button presses to produce the correct joltages
                for (i, joltage) in machine.joltages.iter().enumerate() {
                    let button_press_terms = machine
                        .buttons
                        .iter()
                        .enumerate()
                        .filter_map(|(j, button)| {
                            if button.iter().any(|idx| idx == i) {
                                Some(joltage_terms[j].clone())
                            } else {
                                None
                            }
                        })
                        .collect::<MicroVec<_, 100, u8>>();

                    let lhs = Int::add(&button_press_terms);
                    let rhs = Int::from_u64(*joltage as u64);
                    solver.assert(lhs.eq(&rhs));
                }

                // Use binary search to solve for the lowest button presses
                let objective = Int::add(&joltage_terms);
                let mut lower_bound = 0;
                let mut upper_bound = machine.joltages.iter().sum::<u16>();

                while lower_bound < upper_bound {
                    let mid = (lower_bound + upper_bound) / 2;
                    solver.push();
                    solver.assert(objective.le(Int::from_u64(mid as u64)));

                    match solver.check() {
                        SatResult::Sat => upper_bound = mid,
                        SatResult::Unsat => lower_bound = mid + 1,
                        // Safety: the solver will never fail with the given constraints
                        _ => unsafe { std::hint::unreachable_unchecked() },
                    }

                    solver.pop(1);
                }

                upper_bound
            })
            .sum::<u16>())
    }
}

aoc::solution!();

#[cfg(test)]
mod test {
    use super::{Solution, Solver};

    const INPUT: &str = r"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
";

    #[test]
    fn test_part1() {
        let mut solution = Solution::new(INPUT).unwrap();
        let answer = solution.part1().unwrap().to_string();
        assert_eq!(answer, "7");
    }

    #[test]
    fn test_part2() {
        let mut solution = Solution::new(INPUT).unwrap();
        let answer = solution.part2().unwrap().to_string();
        assert_eq!(answer, "33");
    }
}
