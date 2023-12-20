use std::{fs};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let input_lines: Vec<&str> = input.split("\n\n").collect();
    let seeds: Vec<i64> = input_lines.first().unwrap().split(": ").last().unwrap().split_whitespace().clone().map(|s| s.parse().unwrap()).collect();

    let locations = seeds.iter().map(|s| part1(&s, &input_lines[1..].to_vec()));
    println!("Part1: {}",  locations.min().unwrap());
}

fn part1(seed: &i64, relations: &Vec<& str>) -> i64 {
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

// fn part1(seeds: &Vec<&str>, map: &HashMap<&str, Vec<(i64, i64)>>) -> i64 {
//     let mut lowest_location = i64::MAX;
//     let num_keys = map.keys().len();
//
//     for _s in seeds {
//         let mut curr_key = "seed";
//         let mut s: i64 = _s.parse().unwrap();
//
//         for _i in 0..num_keys {
//             let key = map.keys().find(|s| s.starts_with(curr_key));
//
//             match key {
//                 None => { },
//                 Some(t) => {
//                     curr_key = t.split("-").last().unwrap();
//                     s = map.get(t).unwrap().iter().find(|t| t.0 == s).unwrap_or(&(s, s)).1;
//
//                     if curr_key == "location" {
//                         lowest_location = min(lowest_location, s)
//                     }
//                 }
//             }
//
//
//         }
//
//     }
//
//     return lowest_location
// }