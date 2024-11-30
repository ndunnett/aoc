fn get_unique_packet(signal: &str, length: usize) -> Anyhow<usize> {
    let mut packet = Vec::new();

    for (i, char) in signal.chars().enumerate() {
        packet.push(char);

        while packet.len() > length {
            packet.remove(0);
        }

        let packet_set = packet.iter().collect::<HashSet<_>>();

        if packet.len() == length && packet_set.len() == length {
            return Ok(i + 1);
        }
    }

    Err(anyhow!("failed to find unique packet"))
}

struct Solution {
    input: String,
}

impl Solver for Solution {
    fn new(input: &str) -> Anyhow<Self> {
        Ok(Self {
            input: String::from(input),
        })
    }

    fn part1(&mut self) -> Anyhow<impl fmt::Display> {
        get_unique_packet(&self.input, 4)
    }

    fn part2(&mut self) -> Anyhow<impl fmt::Display> {
        get_unique_packet(&self.input, 14)
    }
}

aoc::solution!();
