#[derive(Clone, Copy)]
#[repr(u8)]
enum Component {
    X = 0,
    M = 1,
    A = 2,
    S = 3,
}

#[derive(Clone, Copy)]
struct ComponentMap<T: Clone + Copy>([T; 4]);

impl<T: Clone + Copy> ComponentMap<T> {
    #[inline(always)]
    fn get(&self, component: &Component) -> T {
        self.0[*component as usize]
    }

    #[inline(always)]
    fn get_mut(&mut self, component: &Component) -> &mut T {
        &mut self.0[*component as usize]
    }
}

impl<T: Clone + Copy> IntoIterator for ComponentMap<T> {
    type Item = T;
    type IntoIter = std::array::IntoIter<Self::Item, 4>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[derive(Clone, Copy)]
struct Range {
    start: u16,
    end: u16,
}

const KEY_VALUES: [Key; u8::MAX as usize + 1] = {
    let mut values = [0; u8::MAX as usize + 1];
    values[b'A' as usize] = 26;
    values[b'R' as usize] = 27;

    let mut i = b'a';

    while i <= b'z' {
        values[i as usize] = (i - b'a' + 1) as Key;
        i += 1;
    }

    values
};

#[inline(always)]
const fn key_from_bytes(bytes: &[u8]) -> Key {
    let mut key = KEY_VALUES[bytes[0] as usize];
    let mut i = 1;

    while i < bytes.len() {
        key = (key << 5) + KEY_VALUES[bytes[i] as usize];
        i += 1;
    }

    key
}

const KEY_A: Key = key_from_bytes(b"A");
const KEY_R: Key = key_from_bytes(b"R");
const KEY_IN: Key = key_from_bytes(b"in");
const KEY_MAX: usize = key_from_bytes(b"zzz") as usize;

type Key = u16;
type Id = u16;
type Cache = KeyMap<Key, Id, KEY_MAX>;
type SourceIndex = KeyMap<Key, usize, KEY_MAX>;
type IdVec = ArrayVec<Id, 4, u8>;

#[derive(Clone, Copy)]
#[repr(C, align(16))]
enum Node {
    Accept,
    Reject,
    Compare {
        left: Component,
        right: u16,
        greater: bool,
        branch: Id,
    },
    Parent(IdVec),
}

#[derive(Clone)]
struct Workflow {
    arena: Vec<Node>,
    in_id: Id,
}

impl Workflow {
    #[inline(always)]
    fn new() -> Self {
        Self {
            arena: Vec::with_capacity(512),
            in_id: 0,
        }
    }

    #[inline(always)]
    fn push(&mut self, workflow: Node) -> Id {
        let id = self.arena.len() as Id;
        self.arena.push(workflow);
        id
    }

    #[inline(always)]
    fn get(&self, id: Id) -> &Node {
        &self.arena[id as usize]
    }

    #[inline(always)]
    fn evaluate(&self, part: &ComponentMap<u16>) -> bool {
        let mut node = self.get(self.in_id);

        loop {
            match node {
                Node::Accept => return true,
                Node::Reject => return false,
                Node::Compare {
                    left,
                    right,
                    greater,
                    branch,
                } => {
                    if (part.get(left) < *right) ^ *greater {
                        node = self.get(*branch);
                    }
                }
                Node::Parent(children) => {
                    for id in children.as_slice() {
                        match self.get(*id) {
                            Node::Compare {
                                left,
                                right,
                                greater,
                                branch,
                            } => {
                                if (part.get(left) < *right) ^ *greater {
                                    node = self.get(*branch);
                                    break;
                                }
                            }
                            child => {
                                node = child;
                                break;
                            }
                        }
                    }
                }
            }
        }
    }

    #[inline(always)]
    fn combinations(&self) -> u64 {
        #[derive(Clone, Copy)]
        struct Frame {
            node_id: Id,
            ranges: ComponentMap<Range>,
        }

        type Stack = ArrayVec<Frame, 16, u8>;

        let mut stack = Stack::new();
        let mut total = 0;

        stack.push(Frame {
            node_id: self.in_id,
            ranges: ComponentMap(
                [Range {
                    start: 1,
                    end: 4000,
                }; 4],
            ),
        });

        while let Some(mut frame) = stack.pop() {
            match self.get(frame.node_id) {
                Node::Accept => {
                    total += frame
                        .ranges
                        .into_iter()
                        .map(|r| (r.end - r.start + 1) as u64)
                        .product::<u64>();
                }
                Node::Reject => {}
                Node::Compare {
                    left,
                    right,
                    greater,
                    branch,
                } => {
                    if *greater {
                        frame.ranges.get_mut(left).start = *right;
                    } else {
                        frame.ranges.get_mut(left).end = right - 1;
                    }

                    frame.node_id = *branch;
                    stack.push(frame);
                }
                Node::Parent(children) => {
                    for id in children.as_slice() {
                        if let Node::Compare {
                            left,
                            right,
                            greater,
                            branch,
                        } = self.get(*id)
                        {
                            let mut branch_ranges = frame.ranges;

                            if *greater {
                                branch_ranges.get_mut(left).start = *right;
                                frame.ranges.get_mut(left).end = *right - 1;
                            } else {
                                branch_ranges.get_mut(left).end = right - 1;
                                frame.ranges.get_mut(left).start = *right;
                            }

                            stack.push(Frame {
                                node_id: *branch,
                                ranges: branch_ranges,
                            });
                        } else {
                            frame.node_id = *id;
                            stack.push(frame);
                        }
                    }
                }
            }
        }

        total
    }
}

/// Assumes that the workflow section ends with a double new line
struct WorkflowParser<'a> {
    bytes: &'a [u8],
    source_index: SourceIndex,
    end: usize,
}

impl<'a> WorkflowParser<'a> {
    fn build_source_index(&mut self) {
        let mut i = 0;

        // Try to parse a valid key
        while let Some(key) = self.key(&mut i) {
            // Index the position following the { after the key
            self.source_index.insert(key, i + 1);

            // Skip through most of the line
            i += 10;

            // Consume characters until the beginning of the next line
            loop {
                match (
                    self.bytes[i],
                    self.bytes[i + 1],
                    self.bytes[i + 2],
                    self.bytes[i + 3],
                ) {
                    (b'\n', _, _, _) => {
                        i += 1;
                        break;
                    }
                    (_, b'\n', _, _) => {
                        i += 2;
                        break;
                    }
                    (_, _, b'\n', _) => {
                        i += 3;
                        break;
                    }
                    (_, _, _, b'\n') => {
                        i += 4;
                        break;
                    }
                    _ => {
                        i += 4;
                    }
                }
            }
        }

        // Skip another new line to mark the end of the workflow section
        self.end = i + 1;
    }

    fn parse(self) -> Anyhow<Workflow> {
        let mut workflow = Workflow::new();
        let mut cache = Cache::new();

        // Seed the index and arena with A and R workflows
        cache.insert(KEY_A, workflow.push(Node::Accept));
        cache.insert(KEY_R, workflow.push(Node::Reject));

        // Parse the "in" node and store the ID
        workflow.in_id = self
            .root(KEY_IN, &mut cache, &mut workflow)
            .ok_or_else(|| anyhow!("failed to parse workflow"))?;

        Ok(workflow)
    }

    fn root(&self, key: Key, cache: &mut Cache, workflow: &mut Workflow) -> Option<Id> {
        if cache.contains_key(key) {
            return cache.get(key).copied();
        }

        let mut i = *self.source_index.get(key)?;
        let mut children = IdVec::new();

        while let Some(child) = self.child(&mut i, cache, workflow) {
            children.push(child);
        }

        let id = workflow.push(Node::Parent(children));
        cache.insert(key, id);
        Some(id)
    }

    fn child(&self, i: &mut usize, cache: &mut Cache, workflow: &mut Workflow) -> Option<Id> {
        let (a, b) = (self.bytes[*i], self.bytes[*i + 1]);

        match (a, b) {
            (b'x' | b'm' | b'a' | b's', b'<' | b'>') => {
                // Parse the component (left hand side of comparison)
                let left = match a {
                    b'x' => Component::X,
                    b'm' => Component::M,
                    b'a' => Component::A,
                    b's' => Component::S,
                    _ => unsafe { std::hint::unreachable_unchecked() },
                };

                // Check the comparison operator
                let greater = b == b'>';

                // Move to the start of the number
                *i += 2;

                // Parse the number (right hand side of comparison)
                let mut right = 0;

                while self.bytes[*i].is_ascii_digit() {
                    right = (right << 1) + (right << 3) + (self.bytes[*i] - b'0') as u16;
                    *i += 1;
                }

                // Add 1 to the number for > conditionals to make them >=, simplifies branching when evaluating nodes
                if greater {
                    right += 1;
                }

                // Consume colon, parse the branch workflow, consume comma
                *i += 1;
                let branch = self.root(self.key(i)?, cache, workflow)?;
                *i += 1;

                Some(workflow.push(Node::Compare {
                    left,
                    right,
                    greater,
                    branch,
                }))
            }
            (b'}' | b'\n', _) => None,
            _ => self.root(self.key(i)?, cache, workflow),
        }
    }

    #[inline(always)]
    fn key(&self, i: &mut usize) -> Option<Key> {
        let mut key = 0;

        loop {
            let val = KEY_VALUES[self.bytes[*i] as usize];

            if val > 0 {
                key = (key << 5) + val;
                *i += 1;
            } else {
                break;
            }
        }

        if key > 0 { Some(key) } else { None }
    }
}

impl<'a> From<&'a str> for WorkflowParser<'a> {
    fn from(value: &'a str) -> Self {
        let mut parser = Self {
            bytes: value.as_bytes(),
            source_index: SourceIndex::new(),
            end: 0,
        };

        parser.build_source_index();
        parser
    }
}

struct PartsParser<'a> {
    number_parser: NumberParser<'a, u16>,
}

impl<'a> PartsParser<'a> {
    fn parse(self) -> Vec<ComponentMap<u16>> {
        self.number_parser
            .tuples()
            .map(|(x, m, a, s)| ComponentMap([x, m, a, s]))
            .collect()
    }
}

impl<'a> From<&WorkflowParser<'a>> for PartsParser<'a> {
    fn from(workflow_parser: &WorkflowParser<'a>) -> Self {
        Self {
            number_parser: NumberParser::<u16>::from(&workflow_parser.bytes[workflow_parser.end..]),
        }
    }
}

#[derive(Clone)]
struct Solution {
    workflow: Workflow,
    parts: Vec<ComponentMap<u16>>,
}

impl Solver for Solution {
    fn new(input: &str) -> Anyhow<Self> {
        let workflow_parser = WorkflowParser::from(input);
        let parts_parser = PartsParser::from(&workflow_parser);

        Ok(Self {
            workflow: workflow_parser.parse()?,
            parts: parts_parser.parse(),
        })
    }

    fn part1(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self
            .parts
            .iter()
            .filter_map(|part| {
                if self.workflow.evaluate(part) {
                    Some(part.into_iter().map(u64::from).sum::<u64>())
                } else {
                    None
                }
            })
            .sum::<u64>())
    }

    fn part2(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self.workflow.combinations())
    }
}

aoc::solution!();

#[cfg(test)]
mod test {
    use super::{Solution, Solver};

    const INPUT: &str = r"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    #[test]
    fn test_part1() {
        let mut solution = Solution::new(INPUT).unwrap();
        let answer = solution.part1().unwrap().to_string();
        assert_eq!(answer, "19114");
    }

    #[test]
    fn test_part2() {
        let mut solution = Solution::new(INPUT).unwrap();
        let answer = solution.part2().unwrap().to_string();
        assert_eq!(answer, "167409079868000");
    }
}
