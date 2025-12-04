#[derive(Clone)]
struct Grid<const N: usize> {
    data: [[u8; N]; N],
}

impl<const N: usize> Grid<N> {
    fn parse(input: &str) -> Self {
        let bytes = input.as_bytes();
        let mut data = [[0; N]; N];

        for y in 0..N - 2 {
            for x in 0..N - 2 {
                data[y + 1][x + 1] = if bytes[y * (N - 1) + x] == b'@' { 1 } else { 0 };
            }
        }

        Self { data }
    }

    fn solve<const REMOVE: bool>(&mut self) -> usize {
        let data = &mut self.data;
        let mut count = 0;

        loop {
            let mut _data = *data;
            let mut removed = 0;

            for y in 1..N - 1 {
                for x in 1..N - 1 {
                    if data[y][x] == 0 {
                        continue;
                    }

                    let neighbours = data[y - 1][x - 1..x + 2].iter().sum::<u8>()
                        + data[y][x - 1..x + 2].iter().sum::<u8>()
                        + data[y + 1][x - 1..x + 2].iter().sum::<u8>();

                    if neighbours < 5 {
                        if REMOVE {
                            _data[y][x] = 0;
                            removed += 1;
                        }

                        count += 1;
                    }
                }
            }

            if REMOVE {
                if removed == 0 {
                    break;
                }

                *data = _data;
            } else {
                break;
            }
        }

        count
    }
}

#[derive(Clone)]
#[repr(transparent)]
struct Solution(Grid<138>); // input is 136*136, +2 for padding

impl Solver for Solution {
    #[inline(always)]
    fn new(input: &str) -> Anyhow<Self> {
        Ok(Self(Grid::parse(input)))
    }

    #[inline(always)]
    fn part1(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self.0.solve::<false>())
    }

    #[inline(always)]
    fn part2(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self.0.solve::<true>())
    }
}

aoc::solution!();

#[cfg(test)]
mod test {
    use super::Grid;

    const INPUT: &str = r"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";

    #[test]
    fn test_part1() {
        assert_eq!(Grid::<12>::parse(INPUT).solve::<false>(), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(Grid::<12>::parse(INPUT).solve::<true>(), 43);
    }
}
