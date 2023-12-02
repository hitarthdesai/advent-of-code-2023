use std::fs;
use std::cmp::max;
const MAX: [i32; 3] = [12, 13, 14];

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let p1: i32 = input.lines().map(part1).sum();
    let p2: i32 = input.lines().map(part2).sum();
    println!("Part1: {p1}");
    println!("Part2: {p2}");
}

fn part1(line: &str) -> i32 {
    let parts = line.split(':');
    let game_id: i32 = parts.clone().nth(0).unwrap().split(' ').nth(1).unwrap().parse().unwrap();
    let sets = parts.clone().nth(1).unwrap().split("; ").map(|s| s.trim().split(", "));

    for set in sets {
        let mut is_set_valid = true;
        for s in set {
            let mut colors = s.split(" ");
            let f: i32 = colors.next().unwrap().parse().unwrap();
            let color = colors.next().unwrap();

            match color {
                "red" => is_set_valid &= f <= MAX[0],
                "green" => is_set_valid &= f <= MAX[1],
                "blue" => is_set_valid &= f <= MAX[2],
                _ => {},
            }

            if is_set_valid == false {
                return 0
            }
        }
    }

    return game_id
}

fn part2(line: &str) -> i32 {
    let parts = line.split(':');
    let game_id: i32 = parts.clone().nth(0).unwrap().split(' ').nth(1).unwrap().parse().unwrap();
    let sets = parts.clone().nth(1).unwrap().split("; ").map(|s| s.trim().split(", "));

    let mut counts = [0, 0, 0];

    for set in sets {
        for s in set {
            let mut colors = s.split(" ");
            let f: i32 = colors.next().unwrap().parse().unwrap();
            let color = colors.next().unwrap();

            match color {
                "red" => counts[0] = max(counts[0], f),
                "green" => counts[1] = max(counts[1], f),
                "blue" => counts[2] = max(counts[2], f),
                _ => {},
            }
        }
    }

    return counts[0] * counts[1] * counts[2]
}
