type Id = u16;

const fn raw_id(bytes: &[u8]) -> usize {
    assert!(bytes.len() == 3);
    let mut id = 0;
    let mut i = 0;

    while i < 3 {
        id = (id << 5) + (bytes[i] - b'a') as usize;
        i += 1;
    }

    id
}

// Raw IDs for key nodes
const DAC_RAW: usize = raw_id(b"dac");
const FFT_RAW: usize = raw_id(b"fft");
const OUT_RAW: usize = raw_id(b"out");
const SVR_RAW: usize = raw_id(b"svr");
const YOU_RAW: usize = raw_id(b"you");

// ID array indices
const DAC: usize = 0;
const FFT: usize = 1;
const OUT: usize = 2;
const SVR: usize = 3;
const YOU: usize = 4;

const EXPANDED_MASK: Id = 1 << 15;
const ID_MASK: Id = EXPANDED_MASK - 1;
const ID_MAX: Id = 560;
const NETWORK_SIZE: usize = ID_MAX as usize + 1;
const TRANSLATION_SIZE: usize = raw_id(b"zzz") + 1;

type IdVec = MicroVec<Id, 25, u8>;

#[derive(Clone)]
struct Solution {
    network: [IdVec; NETWORK_SIZE],
    ids: [Id; 5],
}

impl Solver for Solution {
    fn new(input: &str) -> Anyhow<Self> {
        let mut network = [IdVec::new(); _];
        let mut translation = [ID_MAX; TRANSLATION_SIZE];
        let mut next_id = 0;

        let bytes = input.as_bytes();
        let ptr_range = bytes.as_ptr_range();
        let p_end = ptr_range.end;
        let mut p = ptr_range.start;

        // Safety: beware, raw pointer arithmatic inside
        unsafe {
            while p < p_end {
                // Parse the ID at the beginning of the line
                let mut id = (*p - b'a') as Id;
                p = p.add(1);
                id = (id << 5) + (*p - b'a') as Id;
                p = p.add(1);
                id = (id << 5) + (*p - b'a') as Id;
                p = p.add(2);

                // Add the root ID into the translation table and get a `&mut` to the desitnations vec
                translation[id as usize] = next_id;
                let destinations = &mut network[next_id as usize];
                next_id += 1;

                // Parse remaining IDs on the line into the destinations vec
                while *p != b'\n' {
                    p = p.add(1);
                    let mut dest = (*p - b'a') as Id;
                    p = p.add(1);
                    dest = (dest << 5) + (*p - b'a') as Id;
                    p = p.add(1);
                    dest = (dest << 5) + (*p - b'a') as Id;
                    p = p.add(1);

                    destinations.push_unchecked(dest);
                }

                p = p.add(1);
            }
        }

        // Apply translation to all destinations
        for dests in &mut network {
            for dest in dests {
                *dest = translation[*dest as usize];
            }
        }

        // Apply translation to key nodes
        let mut ids = [0; 5];
        ids[DAC] = translation[DAC_RAW];
        ids[FFT] = translation[FFT_RAW];
        ids[OUT] = translation[OUT_RAW];
        ids[SVR] = translation[SVR_RAW];
        ids[YOU] = translation[YOU_RAW];

        Ok(Self { network, ids })
    }

    fn part1(&mut self) -> Anyhow<impl fmt::Display> {
        // Simple DFS, counting how many paths from "you" reach "out"
        let mut queue = microvec!(IdVec => [self.ids[YOU]]);
        let mut count = 0;

        while let Some(id) = queue.pop() {
            for &dest in &self.network[id as usize] {
                if dest == self.ids[OUT] {
                    count += 1;
                } else {
                    queue.push(dest);
                }
            }
        }

        Ok(count)
    }

    fn part2(&mut self) -> Anyhow<impl fmt::Display> {
        const NEW_CACHE: [u32; NETWORK_SIZE] = [u32::MAX; _];
        let mut stack = MicroVec::<Id, 150, usize>::new();
        let mut count = 1;

        let dfs_points = [
            [self.ids[SVR], self.ids[FFT]],
            [self.ids[FFT], self.ids[DAC]],
            [self.ids[DAC], self.ids[OUT]],
        ];

        // Iterate start and end points ("svr" -> "fft", "fft" -> "dac", "dac" -> "out")
        // Assumes that all paths traverse through "fft" before "dac", may not be true for all inputs
        // Performs cached DFS to count paths
        for [start, target] in dfs_points {
            let mut cache = NEW_CACHE;
            stack.push(start);

            while let Some(state) = stack.pop() {
                let id = state & ID_MASK;

                if id == target {
                    cache[id as usize] = 1;
                    continue;
                }

                if cache[id as usize] != u32::MAX {
                    continue;
                }

                // If the current state hasn't been expanded, requeue it to be reprocessed after exploring child states
                if state & EXPANDED_MASK == 0 {
                    stack.push(state | EXPANDED_MASK);

                    for &next in &self.network[id as usize] {
                        if cache[next as usize] == u32::MAX {
                            stack.push(next);
                        }
                    }
                // The current state is expanded, update cache
                } else {
                    let mut count = 0;

                    for &next in &self.network[id as usize] {
                        count += cache[next as usize];
                    }

                    cache[id as usize] = count;
                }
            }

            count *= cache[start as usize] as u64;
        }

        Ok(count)
    }
}

aoc::solution!();

#[cfg(test)]
mod test {
    use super::{Solution, Solver};

    const INPUT1: &str = r"aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
";

    const INPUT2: &str = r"svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
";

    #[test]
    fn test_part1() {
        let mut solution = Solution::new(INPUT1).unwrap();
        let answer = solution.part1().unwrap().to_string();
        assert_eq!(answer, "5");
    }

    #[test]
    fn test_part2() {
        let mut solution = Solution::new(INPUT2).unwrap();
        let answer = solution.part2().unwrap().to_string();
        assert_eq!(answer, "2");
    }
}
