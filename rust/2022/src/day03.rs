#[allow(dead_code)]
pub fn puzzle() {
    let input = crate::input::load_input(3);
    let mut priorities = 0;

    fn get_priority(char: char) -> i32 {
        if char.is_lowercase() {
            char as i32 - 'a' as i32 + 1
        }
        else {
            char as i32 - 'A' as i32 + 27
        }
    }

    for line in input.lines() {
        let compartments = line.split_at(line.len() / 2);

        for char in compartments.0.chars() {
            if compartments.1.contains(char) {
                priorities += get_priority(char);
                break;
            }
        }
    }

    println!("Part 1 answer: {priorities}");

    priorities = 0;

    for i in (0..input.lines().count()).step_by(3) {
        let line_1 = input.lines().nth(i).unwrap();
        let line_2 = input.lines().nth(i + 1).unwrap();
        let line_3 = input.lines().nth(i + 2).unwrap();

        for char in line_1.chars() {
            if line_2.contains(char) && line_3.contains(char) {
                priorities += get_priority(char);
                break;
            }
        }
    }
    
    println!("Part 2 answer: {priorities}");
}
