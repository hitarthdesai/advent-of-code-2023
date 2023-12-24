// use core::num::dec2flt::parse::parse_number;
use std::{fs};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let _input: Vec<&str> = input.lines().collect();

    let mut series: Vec<Vec<i32>> = Vec::new();
    for i in _input {
        let parts: Vec<i32> = i.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect();
        series.push(parts);
    }

    println!("Part1: {}",  part1(&series));
    // println!("Part2: {}",  part2(&moves, &nodes));
}

fn part1(series: &Vec<Vec<i32>>) -> i32 {
    let mut ans = 0;
    for s in series {
        let mut sub_series: Vec<Vec<i32>> = Vec::new();
        sub_series.push(s.clone());

        let mut is_diff_zero: bool = false;
        while !is_diff_zero {
            let curr_series = sub_series.last().unwrap();
            let mut new_series: Vec<i32> =Vec::new();
            for i in 1..curr_series.len() {
                new_series.push(curr_series[i] - curr_series[i-1])
            }

            sub_series.push(new_series.clone());
            is_diff_zero = new_series.iter().all(|n| *n == 0);
        }

        sub_series.reverse();
        for i in 1..sub_series.len() {
            let mut sum = 0;

            match sub_series.get(i-1) {
                None => { sub_series.get_mut(i).unwrap().push(0) }
                Some(t) => { sum += t.last().unwrap() }
            }
        }

        ans += sub_series.iter().fold(0, |acc, ss| acc + ss.last().unwrap())
    }

    return ans
}

// fn part2(moves: &Vec<char>, nodes: &HashMap<&str, Node>) -> i32 {
//     let mut total_moves:Vec<i32> = Vec::new();
//     let starting_points: Vec<&&str> = nodes.keys().filter(|s| s.ends_with("A")).collect();
//
//     for s in starting_points {
//         let mut num_moves = 0;
//         let mut curr_node = nodes.get(*s).unwrap();
//
//         for _move in moves.iter().cycle() {
//             if curr_node.key.ends_with("Z") { break }
//
//             curr_node = match _move {
//                 'R' => { nodes.get(curr_node.right).unwrap() }
//                 'L' => { nodes.get(curr_node.left).unwrap() }
//                 _t => { curr_node }
//             };
//
//             num_moves += 1;
//         }
//
//         total_moves.push(num_moves)
//     }
//
//     return lcm(&total_moves)
// }
