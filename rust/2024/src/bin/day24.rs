type Symbol = (u8, u8, u8);

#[derive(Clone)]
enum Expr {
    And(Symbol, Symbol),
    Or(Symbol, Symbol),
    Xor(Symbol, Symbol),
}

fn parse_symbol(s: &str) -> Anyhow<Symbol> {
    s.bytes()
        .next_tuple::<Symbol>()
        .ok_or(anyhow!("failed to parse symbol"))
}

fn eval(
    symbol: &Symbol,
    values: &mut FxHashMap<Symbol, bool>,
    exprs: &FxHashMap<Symbol, Expr>,
) -> bool {
    if let Some(&value) = values.get(symbol) {
        return value;
    }

    let value = match exprs[symbol] {
        Expr::And(a, b) => eval(&a, values, exprs) && eval(&b, values, exprs),
        Expr::Or(a, b) => eval(&a, values, exprs) || eval(&b, values, exprs),
        Expr::Xor(a, b) => eval(&a, values, exprs) ^ eval(&b, values, exprs),
    };

    values.insert(*symbol, value);
    value
}

struct Solution {
    values: FxHashMap<Symbol, bool>,
    exprs: FxHashMap<Symbol, Expr>,
}

impl Solution {
    const SPLIT_PATTERN: [char; 3] = ['\n', ':', ' '];
}

impl Solver for Solution {
    fn new(input: &str) -> Anyhow<Self> {
        let (values, exprs) = input
            .split_once("\n\n")
            .ok_or(anyhow!("failed to split input"))?;

        let values = values
            .split(&Self::SPLIT_PATTERN)
            .tuples()
            .map(|(s, _, v)| Ok((parse_symbol(s)?, v != "0")))
            .collect::<Anyhow<_>>()?;

        let exprs = exprs
            .split(&Self::SPLIT_PATTERN)
            .tuples()
            .map(|(a, op, b, _, c)| {
                let mut a = parse_symbol(a)?;
                let mut b = parse_symbol(b)?;
                let c = parse_symbol(c)?;

                if a > b {
                    std::mem::swap(&mut a, &mut b);
                }

                match op {
                    "AND" => Ok((c, Expr::And(a, b))),
                    "OR" => Ok((c, Expr::Or(a, b))),
                    "XOR" => Ok((c, Expr::Xor(a, b))),
                    _ => Err(anyhow!("failed to parse expression")),
                }
            })
            .collect::<Anyhow<_>>()?;

        Ok(Self { values, exprs })
    }

    fn part1(&mut self) -> Anyhow<impl fmt::Display> {
        for symbol in self.exprs.keys() {
            eval(symbol, &mut self.values, &self.exprs);
        }

        Ok(self
            .values
            .iter()
            .filter(|(k, _)| k.0 == b'z')
            .sorted()
            .enumerate()
            .fold(
                0,
                |acc, (i, (_, v))| if *v { acc + 2_u64.pow(i as u32) } else { acc },
            ))
    }

    fn part2(&mut self) -> Anyhow<impl fmt::Display> {
        let bits = self.exprs.keys().filter(|k| k.0 == b'z').count() as u8;
        let first_bit = (b'0', b'0');
        let last_bit = ((bits - 1) / 10 + b'0', (bits - 1) % 10 + b'0');

        Ok(self
            .exprs
            .iter()
            .filter_map(|(wire, expr)| match (wire, expr) {
                // the first 'z' bit should be half adder, so an AND gate is correct
                ((b'z', b'0', b'0'), Expr::And((b'x', b'0', b'0'), (b'y', b'0', b'0'))) => None,
                // the last 'z' bit can be a carry bit, so an OR gate is correct
                ((b'z', z1, z2), Expr::Or(_, _)) if (*z1, *z2) == last_bit => None,
                // all other 'z' bits not produced by XOR gates are invalid
                ((b'z', _, _), Expr::And(_, _) | Expr::Or(_, _)) => Some(wire),
                // output from XOR gates taking an 'x' bit and a 'y' bit should be an operand to an AND gate
                (output, Expr::Xor((b'x', x1, x2), (b'y', y1, y2)))
                    if (*x1, *x2) != first_bit && (*y1, *y2) != first_bit =>
                {
                    if self
                        .exprs
                        .values()
                        .any(|expr| matches!(expr, Expr::And(a, b) if a == output || b == output))
                    {
                        None
                    } else {
                        Some(output)
                    }
                }
                // XOR gates can take an 'x' bit and a 'y' bit to produce a sum intermediate
                (_, Expr::Xor((b'x', _, _), (b'y', _, _))) => None,
                // XOR gates can take intermediate bits to produce a 'z' bit
                ((b'z', _, _), Expr::Xor(_, _)) => None,
                // all other XOR gates are invalid
                (_, Expr::Xor(_, _)) => Some(wire),
                // output from AND gates taking an 'x' bit and a 'y' bit should be an operand to an OR gate
                (output, Expr::And((b'x', x1, x2), (b'y', y1, y2)))
                    if (*x1, *x2) != first_bit && (*y1, *y2) != first_bit =>
                {
                    if self
                        .exprs
                        .values()
                        .any(|expr| matches!(expr, Expr::Or(a, b) if a == output || b == output))
                    {
                        None
                    } else {
                        Some(output)
                    }
                }
                // all other gates are assumed to be correct
                _ => None,
            })
            .sorted()
            .map(|k| format!("{}{}{}", k.0 as char, k.1 as char, k.2 as char))
            .join(","))
    }
}

aoc::solution!();

#[cfg(test)]
mod test {
    use super::{Solution, Solver};

    const INPUT1: &str = r"x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02";

    const INPUT2: &str = r"x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";

    #[test]
    fn test_part1_1() {
        let mut solution = Solution::new(INPUT1).unwrap();
        let answer = solution.part1().unwrap().to_string();
        assert_eq!(answer, "4");
    }

    #[test]
    fn test_part1_2() {
        let mut solution = Solution::new(INPUT2).unwrap();
        let answer = solution.part1().unwrap().to_string();
        assert_eq!(answer, "2024");
    }
}
