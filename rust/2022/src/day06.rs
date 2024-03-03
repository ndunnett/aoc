use std::collections::HashSet;

#[allow(dead_code)]
pub fn puzzle() {
    let input = crate::input::load_input(6);

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

    let mut answer = get_unique_packet(&input, 4);
    println!("Part 1 answer: {}", answer);
    
    answer = get_unique_packet(&input, 14);
    println!("Part 2 answer: {}", answer);
}
