use std::{fs};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let _input: Vec<&str> = input.lines().map(|s| s.split(":").last().unwrap().trim()).collect();
    let times: Vec<u16> = _input.first().unwrap().split_whitespace().map(|x| x.parse().unwrap()).collect();
    let distances: Vec<u16> = _input.last().unwrap().split_whitespace().map(|x| x.parse().unwrap()).collect();

    println!("Part1: {}",  part1(&times, &distances));
    println!("Part2: {}",  part2(&times, &distances));
}

fn part1(times: &Vec<u16>, distances: &Vec<u16>) -> u32 {
    let mut ans: u32 = 1;
    for (i, t) in times.iter().enumerate() {
        let mut num_ways: u32 = 0;
        let record_distance = distances[i];
        for j in 0..*t {
            let speed = j;
            let remaining_time = t - j;
            let distance = speed * remaining_time;

            if distance > record_distance { num_ways += 1 }
        }

        ans *= num_ways;
    }

    return ans
}

fn part2(times: &Vec<u16>, distances: &Vec<u16>) -> u64 {
    let _times: Vec<String> = times.iter().map(|x| x.to_string()).collect();
    let _distances: Vec<String> = distances.iter().map(|x| x.to_string()).collect();

    let t: u64 = _times.join("").parse().unwrap();
    let record_distance: u64 = _distances.join("").parse().unwrap();

    let mut num_ways: u64 = 0;
    for j in 0..t {
        let speed = j;
        let remaining_time = t - j;
        let distance = speed * remaining_time;

        if distance > record_distance { num_ways += 1 }
    }

    return num_ways
}