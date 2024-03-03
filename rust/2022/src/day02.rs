enum Moves {
    Rock,
    Paper,
    Scissors
}

#[allow(dead_code)]
pub fn puzzle() {
    let input = crate::input::load_input(2);
    let mut score = 0;
    let mut opp_move: Moves;
    let mut my_move: Moves;

    for line in input.lines() {
        match line.chars().next().unwrap() {
            'A' => opp_move = Moves::Rock,
            'B' => opp_move = Moves::Paper,
            'C' => opp_move = Moves::Scissors,
            _ => panic!("failed to match opp_move"),
        }

        match line.chars().nth(2).unwrap() {
            'X' => my_move = Moves::Rock,
            'Y' => my_move = Moves::Paper,
            'Z' => my_move = Moves::Scissors,
            _ => panic!("failed to match my_move"),
        }

        match my_move {
            Moves::Rock => {
                match opp_move {
                    Moves::Rock => score += 4,
                    Moves::Paper => score += 1,
                    Moves::Scissors => score += 7,
                }
            },
            Moves::Paper => {
                match opp_move {
                    Moves::Rock => score += 8,
                    Moves::Paper => score += 5,
                    Moves::Scissors => score += 2,
                }
            },
            Moves::Scissors => {
                match opp_move {
                    Moves::Rock => score += 3,
                    Moves::Paper => score += 9,
                    Moves::Scissors => score += 6,
                }
            },
        }
    }

    println!("Part 1 answer: {score}");

    score = 0;

    for line in input.lines() {
        match line.chars().next().unwrap() {
            'A' => opp_move = Moves::Rock,
            'B' => opp_move = Moves::Paper,
            'C' => opp_move = Moves::Scissors,
            _ => panic!("failed to match opp_move"),
        }

        match line.chars().nth(2).unwrap() {
            'X' =>  {
                match opp_move {
                    Moves::Rock => score += 3,
                    Moves::Paper => score += 1,
                    Moves::Scissors => score += 2,
                }
            },
            'Y' => {
                match opp_move {
                    Moves::Rock => score += 4,
                    Moves::Paper => score += 5,
                    Moves::Scissors => score += 6,
                }
            },
            'Z' => {
                match opp_move {
                    Moves::Rock => score += 8,
                    Moves::Paper => score += 9,
                    Moves::Scissors => score += 7,
                }
            },
            _ => panic!("failed to match my_move"),
        }
    }
    
    println!("Part 2 answer: {score}");
}
