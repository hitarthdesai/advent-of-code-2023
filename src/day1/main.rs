use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let p1: i32 = input.lines().map(part1).sum();
    let p2: i32 = input.lines().map(part2).sum();
    println!("Part1: {p1}");
    println!("Part2: {p2}");
}

fn part1(line: &str) -> i32 {
    let d1 = line.chars().find(|c| c.is_numeric()).unwrap_or('0').to_digit(10).unwrap_or(0) as i32;
    let d2 = line.chars().rfind(|c| c.is_numeric()).unwrap_or('0').to_digit(10).unwrap_or(0) as i32;
    return d1 * 10 + d2
}

fn part2(line: &str) -> i32 {
    let line = line
        .replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5e")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e");
    part1(&line)
}