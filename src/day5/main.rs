use std::{fs};
use std::cmp::min;
use std::collections::{HashMap};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let input_lines: Vec<&str> = input.split("\n\n").collect();

    let mut map: HashMap<&str, Vec<(u32, u32)>> = HashMap::new();
    let seeds = parser(&input_lines, &mut map);

    // println!("{:#?}", map.keys());

    let p1: u32 = part1(&seeds, &map);
    println!("Part1: {p1}");
}

fn parser<'a>(lines: &'a Vec<&'a str>, ret: &mut HashMap<&'a str, Vec<(u32, u32)>>) -> Vec<&'a str> {
    let seeds: Vec<&str> = lines.first().unwrap().split(": ").last().unwrap().split_whitespace().clone().collect();

    let relations = &lines[1..].to_vec();
    for r in relations {
        let mut parts = r.split("\n");
        let key = parts.next().unwrap().split_whitespace().next().unwrap();

        ret.insert(key, Vec::new());

        for part in parts {
            let v: Vec<u32> = part.split_whitespace().map(|s| s.parse().unwrap()).collect();
            let mut values = v.iter();
            let d= values.next().unwrap();
            let s= values.next().unwrap();
            let num= values.next().unwrap();


            for i in 0..*num {
                ret.get_mut(key).unwrap().push((s+i, d+i))
            }
        }
    }

    return seeds
}

fn part1(seeds: &Vec<&str>, map: &HashMap<&str, Vec<(u32, u32)>>) -> u32 {
    let mut lowest_location = u32::MAX;
    let num_keys = map.keys().len();

    for _s in seeds {
        let mut curr_key = "seed";
        let mut s: u32 = _s.parse().unwrap();

        for _i in 0..num_keys {
            let key = map.keys().find(|s| s.starts_with(curr_key));

            match key {
                None => { },
                Some(t) => {
                    curr_key = t.split("-").last().unwrap();
                    s = map.get(t).unwrap().iter().find(|t| t.0 == s).unwrap_or(&(s, s)).1;

                    if curr_key == "location" {
                        lowest_location = min(lowest_location, s)
                    }
                }
            }


        }

    }

    return lowest_location
}