aoc::solution!();

fn part1(input: &str) -> i32 {
    let mut answer = 0;

    for line in input.lines() {
        let pairs: Vec<&str> = line.split(',').collect();

        let a: Vec<i32> = pairs[0]
            .split('-')
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        let b: Vec<i32> = pairs[1]
            .split('-')
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        if (a[0] >= b[0] && a[1] <= b[1]) || (a[0] <= b[0] && a[1] >= b[1]) {
            answer += 1;
        }
    }

    answer
}

fn part2(input: &str) -> i32 {
    let mut answer = 0;

    for line in input.lines() {
        let pairs: Vec<&str> = line.split(',').collect();

        let a: Vec<i32> = pairs[0]
            .split('-')
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        let b: Vec<i32> = pairs[1]
            .split('-')
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        if (a[0] <= b[0] || a[1] <= b[1] || a[0] <= b[1])
            && (a[1] >= b[0] || a[0] <= b[1])
            && (a[1] >= b[0] || a[0] >= b[0])
        {
            answer += 1;
        }
    }

    answer
}
