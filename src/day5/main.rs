use std::{fs};
use std::ops::Index;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let input_lines: Vec<&str> = input.split("\n\n").collect();

    let seeds: Vec<i64> = input_lines.first().unwrap().split(": ").last().unwrap().split_whitespace().clone().map(|s| s.parse().unwrap()).collect();
    let locations = seeds.iter().map(|s| part1(&s, &input_lines));
    println!("Part1: {}",  locations.min().unwrap());
    println!("Part2: {}",  part2(&input_lines));

}

fn part1(seed: &i64, input_lines: &Vec<&str>) -> i64 {
    let relations = input_lines[1..].to_vec();
    let mut to_find = *seed;

    for r in relations {
        let ranges = r.split(":\n").last().unwrap();

        for r in ranges.split("\n") {
            let v: Vec<i64> = r.split_whitespace().map(|s| s.parse().unwrap()).collect();
            let mut values = v.iter();
            let d= values.next().unwrap();
            let s= values.next().unwrap();
            let num= values.next().unwrap();

            let offset = to_find - s;
            if offset >= 0 && offset < *num {
                let offset = to_find - s;
                to_find = d + offset;
                break
            }
        }
    }

    return to_find
}

fn part2(input_lines: &Vec<&str>) -> i64 {
    let mut min_location = i64::MAX;
    let ranges: Vec<&str> = input_lines.first().unwrap().split(": ").last().unwrap().split_whitespace().collect();
    let numbers = (0..ranges.len()).step_by(2);

    for i in numbers {
        let range_start: i64 = ranges.clone().index(i).parse().unwrap();
        let count: i64 = ranges.index(i+1).parse().unwrap();

        for j in range_start..range_start+count {
            let location = part1(&j, &input_lines);
            min_location = min_location.min(location);
        };
    }

    return min_location
}