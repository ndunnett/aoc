aoc::solution!();

use std::collections::HashSet;

fn get_unique_packet(signal: &str, length: usize) -> usize {
    let mut packet = Vec::new();

    for (i, char) in signal.chars().enumerate() {
        packet.push(char);

        while packet.len() > length {
            packet.remove(0);
        }

        let packet_set = packet.iter().collect::<HashSet<_>>();

        if packet.len() == length && packet_set.len() == length {
            return i + 1;
        }
    }

    panic!("failed to find unique packet");
}

fn part1(input: &str) -> usize {
    get_unique_packet(input, 4)
}

fn part2(input: &str) -> usize {
    get_unique_packet(input, 14)
}
