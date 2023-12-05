use std::{fmt, fs};
use std::collections::{HashMap, HashSet};
use std::convert::TryInto;
use std::iter::FromIterator;

#[derive(Debug, Copy, Clone)]
struct ScratchCardEntry {
    num_cards: i32,
    num_common: i32
}

impl ScratchCardEntry {
    fn increment_cards(&mut self) {
        self.num_cards += 1;
    }
}

impl fmt::Display for ScratchCardEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Entity {{ num_cards: {}, num_common: {} }}", self.num_cards, self.num_common)
    }
}
fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let input_lines: Vec<&str> = input.lines().collect();

    let p1: i32 = part1(&input_lines);
    let p2: i32 = part2(&input_lines);
    println!("Part1: {p1}");
    println!("Part2: {p2}");
}

fn part1(lines: &Vec<&str>) -> i32 {
    let mut score: i32 = 0;

    for line in lines {
        let mut line_iter = line.split(":").last().unwrap().trim().split("|");
        let winning_numbers: HashSet<&str> = HashSet::from_iter( line_iter.next().unwrap().trim().split_whitespace());
        let my_numbers: HashSet<&str>= HashSet::from_iter( line_iter.next().unwrap().trim().split_whitespace());

        let common_numbers: i32 = winning_numbers.intersection(&my_numbers).count().try_into().unwrap();

        score += match common_numbers {
            0 => { 0 },
            t => { 2_i32.pow((t-1).try_into().unwrap_or(0)) }
        };
    }

    return TryInto::<i32>::try_into(score).unwrap_or(-1)
}
fn part2(lines: &Vec<&str>) -> i32 {
    let mut scratch_cards: HashMap<i32, ScratchCardEntry> = HashMap::new();

    for line in lines {
        let mut line_iter = line.split(":");
        let card_number: i32 = line_iter.next().unwrap().split_ascii_whitespace().last().unwrap().parse().unwrap_or(-1);

        let mut cards_iter = line_iter.next().unwrap().split(":").last().unwrap().trim().split("|");
        let winning_numbers: HashSet<&str> = HashSet::from_iter(cards_iter.next().unwrap().trim().split_whitespace());
        let my_numbers: HashSet<&str> = HashSet::from_iter(cards_iter.next().unwrap().trim().split_whitespace());

        let common_numbers: i32 = winning_numbers.intersection(&my_numbers).count().try_into().unwrap();

        scratch_cards.insert(card_number, ScratchCardEntry {
            num_common: common_numbers,
            num_cards: 1
        });
    }

    let mut total_cards: i32 = 0;
    let number_of_scratch_cards: i32 = scratch_cards.len().try_into().unwrap_or(0);

    for i in 1..number_of_scratch_cards + 1 {
        let card = scratch_cards.get(&i).unwrap();
        let num_cards = card.num_cards;
        let num_common = card.num_common;

        total_cards += num_cards;

        match num_common {
            0 => {},
            _t => {
                for _j in 0..num_cards {
                    for k in i + 1..i + num_common + 1 {
                        match scratch_cards.get_mut(&k) {
                            None => {},
                            Some(t) => t.increment_cards()
                        };
                    }
                }
            }
        }
    }

    return total_cards
}