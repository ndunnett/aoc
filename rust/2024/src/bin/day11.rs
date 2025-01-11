type Cache = FxHashMap<u64, u64>;

fn blink(value: u64, blinks: u64, cache: &mut Cache) -> u64 {
    if blinks == 0 {
        return 1;
    }

    let key = value * 100 + blinks;

    if let Some(&result) = cache.get(&key) {
        return result;
    }

    let result = if value == 0 {
        blink(1, blinks - 1, cache)
    } else {
        let n = value.ilog10() + 1;

        if n % 2 == 0 {
            let m = 10_u64.pow(n / 2);
            blink(value % m, blinks - 1, cache) + blink(value / m, blinks - 1, cache)
        } else {
            blink(value * 2024, blinks - 1, cache)
        }
    };

    cache.insert(key, result);
    result
}

#[derive(Clone)]
struct Solution {
    stones: Vec<u64>,
    cache: Cache,
}

impl Solution {
    fn solve(&mut self, blinks: u64) -> u64 {
        self.stones
            .iter()
            .map(|&value| blink(value, blinks, &mut self.cache))
            .sum()
    }
}

impl Solver for Solution {
    fn new(input: &str) -> Anyhow<Self> {
        Ok(Self {
            stones: input
                .trim()
                .split(' ')
                .map(str::parse::<u64>)
                .collect::<ParseIntResult<Vec<_>>>()?,
            cache: Cache::with_capacity_and_hasher(150000, FxBuildHasher),
        })
    }

    fn part1(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self.solve(25))
    }

    fn part2(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self.solve(75))
    }
}

aoc::solution!();

#[cfg(test)]
mod test {
    use super::{Solution, Solver};

    const INPUT: &str = r"125 17";

    #[test]
    fn test_part1() {
        let mut solution = Solution::new(INPUT).unwrap();
        let answer = solution.part1().unwrap().to_string();
        assert_eq!(answer, "55312");
    }

    // no test given for part 2
}
