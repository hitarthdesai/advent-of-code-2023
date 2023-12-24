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
    println!("Part2: {}",  part2(&series));
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

        ans += sub_series.iter().fold(0, |acc, ss| acc + ss.last().unwrap())
    }

    return ans
}
fn part2(series: &Vec<Vec<i32>>) -> i32 {
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
        ans += sub_series.iter().fold(0, |acc, ss| ss.first().unwrap() - acc)
    }

    return ans
}
