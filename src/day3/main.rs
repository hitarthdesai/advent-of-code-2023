use std::fs;
use std::fmt;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

#[derive(Debug, Copy, Clone)]
struct Entity {
    start: usize,
    end: usize,
    number: i64
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

    let p1: i64 = part1(&engine, input_lines.clone());
    let p2: i64 = part2(&engine, input_lines);
    println!("Part1: {p1}");
    println!("Part2 (Incorrect): {p2}");
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
                        number: c.to_digit(10).unwrap() as i64
                    }),
                    Some(t) => {
                        if i == t.end + 1 {
                            t.end += 1;
                            t.number = t.number * 10 + c.to_digit(10).unwrap() as i64
                        } else {
                            parsed_line.push(Entity {
                                start: i,
                                end: i,
                                number: c.to_digit(10).unwrap() as i64
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

fn part1(engine: &Vec<Vec<Entity>>, lines: Vec<&str>) -> i64 {
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

        let length = bounding_box.last().unwrap().len();

        for part in engine_line {
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

            sum += match chars_to_check.chars().any(is_special_character) {
                true => part.number,
                false => 0
            };
        }
    }

    return sum
}

fn part2(engine: &Vec<Vec<Entity>>, lines: Vec<&str>) -> i64 {
    let max_x = lines.first().unwrap().len();
    let max_y = lines.len();
    let mut gears: HashMap<(usize, usize), Vec<Entity>> = HashMap::new();

    let mut sum: i64 = 0;
    for (i, line) in lines.iter().enumerate() {
        line.chars().enumerate().for_each(|(j, c)| {
            if c == '*' {
                let i_range = match i {
                    0 => [0, 1, 2*max_x],
                    t => [t-1, t, t+1]
                };

                let j_range = match j {
                    0 => [0, 1, 2*max_y],
                    t => [t-1, t, t+1]
                };

                let mut valid_surrounding: Vec<(usize, usize)> = Vec::new();
                for _i in i_range {
                    for _j in j_range {
                        if _i >= 0 && _i < max_x && _j > 0 && _j < max_y && !(_i == i && _j == j) {
                            valid_surrounding.push((_i, _j));
                        }
                    }
                }

                for (r, c) in valid_surrounding {
                    for part in engine.get(r).unwrap() {
                        if part.start+1 <= c && c <= part.end+1 {
                            match gears.get(&(i, j)) {
                                None => {
                                    let mut vec: Vec<Entity> = Vec::new();
                                    vec.push(*part);
                                    gears.insert((i, j), vec);
                                },
                                Some(t) => {
                                    let mut does_exist = false;
                                    t.iter().for_each(|_t| {
                                        if part.start == _t.start && part.end == _t.end && part.number == _t.number {
                                            does_exist = true
                                        }
                                    });

                                    if !does_exist {
                                       gears.entry((i, j)).and_modify(|t| t.push(*part));
                                    }
                                }
                            };
                        }
                    }
                }
            }
        });
    }

    for entities in gears.values() {
        if entities.len() != 2 {
            continue
        }

        let mut product: i64 = 1;
        for e in entities {
            product *= e.number;
        }

        sum += product;
    }

    return sum
}
