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
