struct Solution {
    connections: FxHashMap<(u8, u8), FxHashSet<(u8, u8)>>,
}

impl Solver for Solution {
    fn new(input: &str) -> Anyhow<Self> {
        let mut connections = FxHashMap::default();

        for (a0, a1, _, b0, b1, _) in input.bytes().tuples() {
            connections
                .entry((a0, a1))
                .or_insert(FxHashSet::default())
                .insert((b0, b1));

            connections
                .entry((b0, b1))
                .or_insert(FxHashSet::default())
                .insert((a0, a1));
        }

        Ok(Self { connections })
    }

    fn part1(&mut self) -> Anyhow<impl fmt::Display> {
        let mut interconnected = FxHashSet::default();

        for (a, connections) in self.connections.iter() {
            if a.0 != b't' {
                continue;
            }

            for b in connections.iter() {
                for c in connections.intersection(&self.connections[b]) {
                    let (x, y, z) = [a, b, c]
                        .into_iter()
                        .sorted()
                        .next_tuple()
                        .ok_or(anyhow!("failed to sort group of three"))?;

                    interconnected.insert((x, y, z));
                }
            }
        }

        Ok(interconnected.len())
    }

    fn part2(&mut self) -> Anyhow<impl fmt::Display> {
        let mut cliques = self
            .connections
            .keys()
            .map(|&pc| FxHashSet::from_iter([pc]))
            .collect::<Vec<_>>();

        for clique in cliques.iter_mut() {
            for a in self.connections.keys() {
                if clique.iter().all(|b| self.connections[a].contains(b)) {
                    clique.insert(*a);
                }
            }
        }

        Ok(cliques
            .iter()
            .max_by(|a, b| a.len().cmp(&b.len()))
            .ok_or(anyhow!("failed to find biggest clique"))?
            .iter()
            .sorted()
            .map(|node| format!("{}{}", node.0 as char, node.1 as char))
            .join(","))
    }
}

aoc::solution!();

#[cfg(test)]
mod test {
    use super::{Solution, Solver};

    const INPUT: &str = r"kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
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
        assert_eq!(answer, "co,de,ka,ta");
    }
}
