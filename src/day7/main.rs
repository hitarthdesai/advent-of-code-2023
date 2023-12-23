use std::{fmt, fs};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::convert::TryInto;

#[derive(Debug, Copy, Clone)]
struct Hand<'a> {
    cards: &'a str,
    bid: u32,
    rank: u32,
}

impl fmt::Display for Hand<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Entity {{ cards: {}, bid: {}, rank: {} }}", self.cards, self.bid, self.rank)
    }
}

impl Hand<'_> {
    fn get_frequency_map(&self) -> HashMap<char, u8> {
        let mut letter_counts: HashMap<char, u8> = HashMap::new();

        let v: Vec<char> = self.cards.to_lowercase().chars().collect();
        for c in v {
            *letter_counts.entry(c).or_insert(0) += 1;
        }

        return letter_counts
    }

    // 1: Five of a kind, where all five cards have the same label: AAAAA
    // 2: Four of a kind, where four cards have the same label and one card has a different label: AA8AA
    // 3: Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
    // 4: Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
    // 5: Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
    // 6: One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
    // 7: High card, where all cards' labels are distinct: 23456
    fn get_strength_by_rule_one(&self) -> u8 {

        let f: HashMap<char, u8> = self.get_frequency_map();
        let num_cards_in_map = f.len();

        return match num_cards_in_map {
            1 => { 1 }
            2 => {
                if f.values().max().unwrap() == &4 { 2 }
                else { 3 }
            }
            3 => {
                if f.values().max().unwrap() == &3 { 4 }
                else { 5 }
            }
            4 => { 6 }
            5 => { 7 }
            _ => { 0 }
        }
    }
}

fn card_to_int(c: &char, is_part_two: bool) -> u32 {
    match c {
        'A' => { 14 },
        'K' => { 13 },
        'Q' => { 12 },
        'J' => {
            if is_part_two { 1 }
            else { 11 }
        },
        'T' => { 10 },
        c => { c.to_digit(10).unwrap() },
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let _input: Vec<&str> = input.lines().collect();

    let mut hands: HashMap<&str, Hand> = HashMap::new();

    for hand in _input {
        let _hand: Vec<&str> = hand.split_whitespace().collect();
        let cards = _hand.first().unwrap();
        let bid: u32 = _hand.last().unwrap().parse().unwrap();

        hands.insert(cards, Hand {
            cards,
            bid,
            rank: u32::MAX
        });
    }

    println!("Part1: {}",  part1(&mut hands));
    // println!("Part2: {}",  part2(&times, &distances));
}

fn part1(hands: &mut HashMap<&str, Hand>) -> u32 {
    let mut score: u32 = 0;
    let mut hands_by_strength: HashMap<u8, Vec<&mut Hand>> = HashMap::new();
    let mut current_rank: u32 = hands.len().try_into().unwrap();

    for h in hands.values_mut() {
        let strength = h.get_strength_by_rule_one();

        if hands_by_strength.contains_key(&strength) {
            hands_by_strength.get_mut(&strength).unwrap().push(h)
        } else {
            let mut v: Vec<&mut Hand> = Vec::new();
            v.push(h);
            hands_by_strength.insert(strength, v);
        }
    }


    for i in 1..8 {
        if !hands_by_strength.contains_key(&i) { continue }

        let _hands = hands_by_strength.get_mut(&i).unwrap();
        _hands.sort_by(|x, y| {
            for (i, c1) in x.cards.chars().enumerate() {
                let c2 = y.cards.chars().nth(i).unwrap();

                let _c1 = card_to_int(&c1, false);
                let _c2 = card_to_int(&c2, false);

                match _c1.cmp(&_c2) {
                    Ordering::Equal => { continue }
                    t => { return t.reverse() }
                }
            }

            return Ordering::Equal
        });

        for h in _hands {
            h.rank = current_rank;
            score += current_rank * h.bid;
            current_rank -= 1;
        }
    }

    // println!("{:#?}", hands_by_strength);
    // println!("{:#?}", hands.values().cycle());
    return score
}

// fn part2(times: &Vec<u32>, distances: &Vec<u32>) -> u64 {
//     let _times: Vec<String> = times.iter().map(|x| x.to_string()).collect();
//     let _distances: Vec<String> = distances.iter().map(|x| x.to_string()).collect();
//
//     let t: u64 = _times.join("").parse().unwrap();
//     let record_distance: u64 = _distances.join("").parse().unwrap();
//
//     let mut num_ways: u64 = 0;
//     for j in 0..t {
//         let speed = j;
//         let remaining_time = t - j;
//         let distance = speed * remaining_time;
//
//         if distance > record_distance { num_ways += 1 }
//     }
//
//     return num_ways
// }