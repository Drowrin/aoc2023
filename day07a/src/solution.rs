use itertools::Itertools;
use std::collections::HashMap;

fn get_val(c: u8) -> u8 {
    match c {
        b'A' => 14,
        b'K' => 13,
        b'Q' => 12,
        b'J' => 11,
        b'T' => 10,
        b'9' => 9,
        b'8' => 8,
        b'7' => 7,
        b'6' => 6,
        b'5' => 5,
        b'4' => 4,
        b'3' => 3,
        b'2' => 2,
        _ => panic!("Not a card character: {c}"),
    }
}

fn get_strength(hand: &[u8]) -> u8 {
    let mut cards = HashMap::<u8, u8>::new();

    for card in hand {
        cards.entry(*card).and_modify(|e| *e += 1).or_insert(1);
    }

    let mut best = cards.values().sorted().rev();

    match best.next().unwrap() {
        5 => 6,
        4 => 5,
        3 => match best.next().unwrap() {
            2 => 4,
            _ => 3,
        },
        2 => match best.next().unwrap() {
            2 => 2,
            _ => 1,
        },
        _ => 0,
    }
}

fn parse_hand(hand: &str) -> [u8; 6] {
    let bytes = hand.as_bytes();
    [
        get_strength(bytes),
        get_val(bytes[0]),
        get_val(bytes[1]),
        get_val(bytes[2]),
        get_val(bytes[3]),
        get_val(bytes[4]),
    ]
}

pub fn solution(input: &str) -> impl ToString {
    input
        .split("\n")
        .map(|line| line.split_whitespace().collect_tuple().unwrap())
        .map(|(hand, bid)| (parse_hand(hand), bid.parse::<usize>().unwrap()))
        .sorted_by_key(|(hand, _)| *hand)
        .enumerate()
        .map(|(i, (_, bid))| (1 + i) * bid)
        .sum::<usize>()
}
