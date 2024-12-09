// this is really bad, need to completely rewrite with different ideas

use std::cmp::Ordering;

#[derive(Clone, Copy, PartialEq)]
enum Space {
    File(usize, u8),
    Free(u8),
}

struct FileSystem {
    space: Vec<Space>,
}

impl From<&str> for FileSystem {
    fn from(input: &str) -> Self {
        Self {
            space: input
                .trim()
                .bytes()
                .enumerate()
                .filter_map(|(i, n)| {
                    if i % 2 == 0 {
                        Some(Space::File(i / 2, n - b'0'))
                    } else if n > b'0' {
                        Some(Space::Free(n - b'0'))
                    } else {
                        None
                    }
                })
                .collect(),
        }
    }
}

impl FileSystem {
    fn iter(&self) -> FileSystemIterator<'_> {
        FileSystemIterator::from(self)
    }

    fn iter_part1(&self) -> Part1Iterator<'_> {
        Part1Iterator::from(self)
    }

    fn compact_unfragmented(&self) -> Self {
        let mut space = self.space.clone();
        let mut i = space.len() - 1;

        while i > 0 {
            if let Space::File(_, size) = space[i] {
                for j in 0..i {
                    if let Space::Free(free) = space[j] {
                        match free.cmp(&size) {
                            Ordering::Greater => {
                                space[j] = Space::Free(free - size);
                                space.insert(j, space[i]);
                                i += 1;
                                space[i] = Space::Free(size);
                                break;
                            }
                            Ordering::Equal => {
                                space[j] = space[i];
                                space[i] = Space::Free(size);
                                break;
                            }
                            Ordering::Less => {}
                        }
                    }
                }
            }

            i -= 1;
        }

        Self { space }
    }
}

#[derive(Clone, Copy, PartialEq)]
struct Fragment<'a> {
    space: &'a Space,
    index: usize,
    taken: u8,
}

struct Part1Iterator<'a> {
    space: &'a Vec<Space>,
    front: Fragment<'a>,
    back: Fragment<'a>,
}

impl<'a> From<&'a FileSystem> for Part1Iterator<'a> {
    fn from(fs: &'a FileSystem) -> Self {
        Self {
            space: &fs.space,
            front: Fragment {
                space: &fs.space[0],
                index: 0,
                taken: 0,
            },
            back: Fragment {
                space: &fs.space[fs.space.len() - 1],
                index: fs.space.len() - 1,
                taken: 0,
            },
        }
    }
}

impl Iterator for Part1Iterator<'_> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match (self.front.space, self.back.space) {
                (Space::File(front_id, front_size), _) => {
                    if self.front.taken < *front_size {
                        // can take from front
                        self.front.taken += 1;
                        return Some(*front_id);
                    } else if self.front.index + 1 < self.back.index {
                        // can increment front and loop again
                        self.front.taken = 0;
                        self.front.index += 1;
                        self.front.space = &self.space[self.front.index];
                    } else if let Space::File(back_id, back_size) = self.back.space {
                        if self.back.taken < *back_size {
                            // front exhausted, can take from back
                            self.back.taken += 1;
                            return Some(*back_id);
                        } else {
                            return None;
                        }
                    } else {
                        return None;
                    }
                }
                (Space::Free(front_size), Space::File(back_id, back_size)) => {
                    if self.front.taken < *front_size {
                        if self.back.taken < *back_size {
                            // space at front, can take from back
                            self.front.taken += 1;
                            self.back.taken += 1;
                            return Some(*back_id);
                        } else if self.back.index - 1 > self.front.index {
                            // space at front, can increment back and loop again
                            self.back.taken = 0;
                            self.back.index -= 1;
                            self.back.space = &self.space[self.back.index];
                        } else {
                            return None;
                        }
                    } else if self.front.index + 1 < self.back.index {
                        // can increment front and loop again
                        self.front.taken = 0;
                        self.front.index += 1;
                        self.front.space = &self.space[self.front.index];
                    } else if self.back.taken < *back_size {
                        // front exhausted, can take from back
                        self.back.taken += 1;
                        return Some(*back_id);
                    } else {
                        return None;
                    }
                }
                (Space::Free(..), Space::Free(..)) => {
                    if self.back.index - 1 > self.front.index {
                        // can increment back and loop again
                        self.back.taken = 0;
                        self.back.index -= 1;
                        self.back.space = &self.space[self.back.index];
                    } else {
                        return None;
                    }
                }
            }
        }
    }
}

struct FileSystemIterator<'a> {
    fs: &'a FileSystem,
    index: usize,
    taken: u8,
}

impl<'a> From<&'a FileSystem> for FileSystemIterator<'a> {
    fn from(fs: &'a FileSystem) -> Self {
        Self {
            fs,
            index: 0,
            taken: 0,
        }
    }
}

impl Iterator for FileSystemIterator<'_> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let (id, size) = match self.fs.space[self.index] {
            Space::File(id, size) => (id, size),
            Space::Free(size) => (0, size),
        };

        if self.taken < size {
            self.taken += 1;
            Some(id)
        } else if self.index + 1 < self.fs.space.len() {
            self.index += 1;
            self.taken = 1;

            match self.fs.space[self.index] {
                Space::File(id, _) => Some(id),
                Space::Free(..) => Some(0),
            }
        } else {
            None
        }
    }
}

struct Solution {
    fs: FileSystem,
}

impl Solver for Solution {
    fn new(input: &str) -> Anyhow<Self> {
        Ok(Self {
            fs: FileSystem::from(input),
        })
    }

    fn part1(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self
            .fs
            .iter_part1()
            .enumerate()
            .map(|(i, id)| i * id)
            .sum::<usize>())
    }

    fn part2(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self
            .fs
            .compact_unfragmented()
            .iter()
            .enumerate()
            .map(|(i, id)| i * id)
            .sum::<usize>())
    }
}

aoc::solution!();

#[cfg(test)]
mod test {
    use super::{Solution, Solver};

    // don't forget about the trailing new line
    const INPUT: &str = r"2333133121414131402
";

    #[test]
    fn test_part1() {
        let mut solution = Solution::new(INPUT).unwrap();
        let answer = solution.part1().unwrap().to_string();
        assert_eq!(answer, "1928");
    }

    #[test]
    fn test_part2() {
        let mut solution = Solution::new(INPUT).unwrap();
        let answer = solution.part2().unwrap().to_string();
        assert_eq!(answer, "2858");
    }
}
