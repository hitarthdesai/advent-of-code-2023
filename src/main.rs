use std::fs;

fn main() {
    let input = fs::read_to_string("./src/input.txt").unwrap();
    let p1: i32 = input.lines().map(p1_calibration_value).sum();
    let p2: i32 = input.lines().map(p2_calibration_value).sum();
    println!("Part1: {p1}");
    println!("Part2: {p2}");
}

fn p1_calibration_value(line: &str) -> i32 {
    let d1 = line.chars().find(|c| c.is_numeric()).unwrap_or('0').to_digit(10).unwrap_or(0) as i32;
    let d2 = line.chars().rfind(|c| c.is_numeric()).unwrap_or('0').to_digit(10).unwrap_or(0) as i32;
    return d1 * 10 + d2
}

fn p2_calibration_value(line: &str) -> i32 {
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
    p1_calibration_value(&line)
}