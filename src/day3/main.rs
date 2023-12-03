use std::fs;
use std::fmt;

#[derive(Debug)]
struct Entity {
    start: usize,
    end: usize,
    number: i32
}

impl fmt::Display for Entity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Entity {{ start: {}, end: {}, number: {} }}", self.start, self.end, self.number)
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let input_lines: Vec<&str> = input.lines().collect();
    let engine = parse_engine(&input);

    let p1: i32 = part1(engine, input_lines);
    println!("Part1: {p1}");
    // let p2: i32 = input.lines().map(part2).sum();
    // println!("Part2: {p2}");
}

fn parse_engine(engine: &String) -> Vec<Vec<Entity>> {
    let mut parsed_engine: Vec<Vec<Entity>> = Vec::new();
    engine.lines().for_each(|l| {
        let mut parsed_line: Vec<Entity> = Vec::new();

        l.chars().enumerate().for_each(|(i, c)| {
            if c.is_digit(10) {
                match parsed_line.last_mut() {
                    None => parsed_line.push(Entity {
                        start: i,
                        end: i,
                        number: c.to_digit(10).unwrap() as i32
                    }),
                    Some(t) => {
                        if i == t.end + 1 {
                            t.end += 1;
                            t.number = t.number * 10 + c.to_digit(10).unwrap() as i32
                        } else {
                            parsed_line.push(Entity {
                                start: i,
                                end: i,
                                number: c.to_digit(10).unwrap() as i32
                            })
                        }
                    }
                }
            }
        });

        parsed_engine.push(parsed_line);
    });

    parsed_engine
}

fn is_special_character(c: char) -> bool {
    !c.is_digit(10) && c != '.'
}

fn search_bounding_box(bounding_box: &[&str], i: usize, part: &Entity) -> i32 {
    let length = bounding_box.last().unwrap().len();
    let start = part.start;
    let end = part.end;

    let mut chars_to_check = String::new();
    match bounding_box.len() {
        2 => {
            if i == 0 {
                chars_to_check.push_str(&bounding_box.last().unwrap()[start..end+1]);
            } else {
                chars_to_check.push_str(&bounding_box.first().unwrap()[start..end+1]);
            }

            if start > 0 {
                chars_to_check.push_str(bounding_box.first().unwrap().get(start-1..start).unwrap_or("."));
                chars_to_check.push_str(bounding_box.last().unwrap().get(start-1..start).unwrap_or("."));
            }

            if end < length - 1 {
                chars_to_check.push_str(bounding_box.first().unwrap().get(end+1..).unwrap_or(".").get(0..1).unwrap());
                chars_to_check.push_str(bounding_box.last().unwrap().get(end+1..).unwrap_or(".").get(0..1).unwrap());
            }
        },
        3 => {
            chars_to_check.push_str(&bounding_box.first().unwrap()[start..end+1]);
            chars_to_check.push_str(&bounding_box.get(1..2).unwrap().first().unwrap()[start..end+1]);
            chars_to_check.push_str(&bounding_box.last().unwrap()[start..end+1]);

            if start > 0 {
                chars_to_check.push_str(bounding_box.first().unwrap().get(start-1..start).unwrap_or("."));
                chars_to_check.push_str(bounding_box.get(1..2).unwrap().first().unwrap().get(start-1..start).unwrap_or("."));
                chars_to_check.push_str(bounding_box.last().unwrap().get(start-1..start).unwrap_or("."));
            }
            if end < length - 1 {
                chars_to_check.push_str(bounding_box.first().unwrap().get(end+1..).unwrap_or(".").get(0..1).unwrap());
                chars_to_check.push_str(bounding_box.get(1..2).unwrap().first().unwrap().get(end+1..).unwrap_or(".").get(0..1).unwrap());
                chars_to_check.push_str(bounding_box.last().unwrap().get(end+1..).unwrap_or(".").get(0..1).unwrap());
            }
        },
        _ => {}
    }

    match chars_to_check.chars().any(is_special_character) {
        true => part.number,
        false => 0
    }

}

fn part1(engine: Vec<Vec<Entity>>, lines: Vec<&str>) -> i32 {
    let max = engine.len();
    let mut sum = 0;

    for (i, engine_line) in engine.iter().enumerate() {
        let bounding_box = match i {
            0 => &lines[0..2],
            t => {
                if t == max - 1 {
                    &lines[t-1..]
                } else if t == max - 2 {
                    &lines[t-1..]
                } else {
                    &lines[t-1..t+2]
                }
            },
        };

        for part in engine_line {
            sum += search_bounding_box(bounding_box, i, part);
        }
    }

    return sum
}

// fn part2(line: &str) -> i32 {
//     let parts = line.split(':');
//     let game_id: i32 = parts.clone().nth(0).unwrap().split(' ').nth(1).unwrap().parse().unwrap();
//     let sets = parts.clone().nth(1).unwrap().split("; ").map(|s| s.trim().split(", "));
//
//     let mut counts = [0, 0, 0];
//
//     for set in sets {
//         for s in set {
//             let mut colors = s.split(" ");
//             let f: i32 = colors.next().unwrap().parse().unwrap();
//             let color = colors.next().unwrap();
//
//             match color {
//                 "red" => counts[0] = max(counts[0], f),
//                 "green" => counts[1] = max(counts[1], f),
//                 "blue" => counts[2] = max(counts[2], f),
//                 _ => {},
//             }
//         }
//     }
//
//     return counts[0] * counts[1] * counts[2]
// }
