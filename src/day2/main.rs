use std::fs;
const MAX: [i32; 3] = [12, 13, 14];

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let p1: i32 = input.lines().map(part1).sum();
    println!("Part1: {p1}");
    // let p2: i32 = input.lines().map(p2_calibration_value).sum();
    // println!("Part2: {p2}");
}

fn part1(line: &str) -> i32 {
    let parts = line.split(':');
    let game_id = parts.clone().nth(0).unwrap().split(' ').nth(1).unwrap();
    let sets = parts.clone().nth(1).unwrap().split("; ").map(|s| s.trim().split(", "));

    let mut counts = [0, 0, 0];
    for set in sets {
        for s in set {
            let mut colors = s.split(" ");
            let f: i32 = colors.next().unwrap().parse().unwrap();
            let color = colors.next().unwrap();

            match color {
                "red" => counts[0] += f,
                "green" => counts[1] += f,
                "blue" => counts[2] += f,
                _ => {},
            }
        }
    }

    let is_valid = counts.iter().enumerate().all(|(i, n)| n <= &MAX[i]);

    let return_val: i32 = match is_valid {
        true => game_id.parse().unwrap(),
        false => 0
    };

    return_val
}
