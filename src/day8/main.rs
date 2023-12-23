use std::{fmt, fs};
use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
struct Node<'a> {
    key: &'a str,
    left: &'a str,
    right: &'a str
}

impl fmt::Display for Node<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Entity {{ key: {}\nleft: {:#?}\nright: {:#?} }}", self.key, self.left, self.right)
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let _input: Vec<&str> = input.lines().collect();

    let moves: Vec<char> = _input.first().unwrap().chars().collect();
    let _nodes = _input[2..].to_vec();

    let mut nodes: HashMap<&str, Node> = HashMap::new();

    for node in _nodes {
        let mut parts = node.split(" = ");
        let key = parts.next().unwrap();
        let mut rem = parts.next().unwrap().split(", ");
        let left = &rem.next().unwrap()[1..];
        let right = &rem.next().unwrap()[..3];

        nodes.insert(key, Node {
            key,
            left,
            right
        });
    }

    println!("Part1: {}",  part1(&moves, &nodes));
    println!("Part2: {}",  part2(&moves, &nodes));
}

fn part1(moves: &Vec<char>, nodes: &HashMap<&str, Node>) -> u64 {
    let mut curr_node = nodes.get("AAA").unwrap();
    let mut num_moves = 0;

    for _move in moves.iter().cycle() {
        if curr_node.key == "ZZZ" { break }

        curr_node = match _move {
            'R' => { nodes.get(curr_node.right).unwrap() }
            'L' => { nodes.get(curr_node.left).unwrap() }
            _t => { curr_node }
        };

        num_moves += 1;
    }

    return num_moves
}

fn lcm(nums: &Vec<u64>) -> u64 {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..].to_vec());
    a * b / gcd(a, b)
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

fn part2(moves: &Vec<char>, nodes: &HashMap<&str, Node>) -> u64 {
    let mut total_moves:Vec<u64> = Vec::new();
    let starting_points: Vec<&&str> = nodes.keys().filter(|s| s.ends_with("A")).collect();

    for s in starting_points {
        let mut num_moves = 0;
        let mut curr_node = nodes.get(*s).unwrap();

        for _move in moves.iter().cycle() {
            if curr_node.key.ends_with("Z") { break }

            curr_node = match _move {
                'R' => { nodes.get(curr_node.right).unwrap() }
                'L' => { nodes.get(curr_node.left).unwrap() }
                _t => { curr_node }
            };

            num_moves += 1;
        }

        total_moves.push(num_moves)
    }

    return lcm(&total_moves)
}
