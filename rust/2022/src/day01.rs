#[allow(dead_code)]
pub fn puzzle() {
    let input = crate::input::load_input(1);
    let mut reindeer = Vec::new();
    let mut current = 0;

    for line in input.lines() {
        if line.is_empty() {
            reindeer.push(current);
            current = 0;
        }
        else {
            current += line.trim().parse::<i32>().unwrap();
        }
    }

    println!("Part 1 answer: {:?}", reindeer.iter().max().unwrap());

    reindeer.sort_by(|b, a| a.partial_cmp(b).unwrap());
    println!("Part 2 answer: {}", reindeer[0..3].iter().sum::<i32>());
}
