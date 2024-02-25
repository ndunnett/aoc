from mojo import runner, fmt


fn get_digits(line: String) raises -> Int:
    var num = String()

    for j in range(len(line)):
        let ch = ord(line[j])

        if isdigit(ch):
            num += line[j]
            break

    for j in range(len(line)):
        let ch = ord(line[len(line) - 1 - j])

        if isdigit(ch):
            num += line[len(line) - 1 - j]
            break

    return atol(num)


fn part1(input: String) raises -> String:
    let lines = input.split("\n")
    var sum = 0

    for i in range(len(lines)):
        sum += get_digits(lines[i])

    return sum


fn part2(input: String) raises -> String:
    let lines = input.split("\n")
    var sum = 0

    for i in range(len(lines)):
        sum += get_digits(
            lines[i]
            .replace("one", "o1e")
            .replace("two", "t2o")
            .replace("three", "t3e")
            .replace("four", "f4r")
            .replace("five", "f5e")
            .replace("six", "s6x")
            .replace("seven", "s7n")
            .replace("eight", "e8t")
            .replace("nine", "n9e")
        )

    return sum


fn main() raises:
    runner("Part 1 answer:", 2023, 1, part1)
    runner("Part 2 answer:", 2023, 1, part2)
