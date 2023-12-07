use std::cmp::Ordering;
use std::collections::HashMap;

use itertools::Itertools;

use crate::day7::HandType::{FiveOfAKind, FourOfAKind, FullHouse, HighCard, OnePair, ThreeOfAKind, TwoPairs};
use crate::input_reader::read_lines;

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord)]
#[repr(usize)]
enum HandType {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Hand {
    cards: Vec<char>,
    bid: usize,
    hand_type: HandType,
}

impl Hand {
    fn new(hand: &str) -> Self {
        Self::build_with(hand, find_hand_type)
    }

    fn build_with(hand: &str, find_hand_type: fn(HashMap<&char, usize>) -> HandType) -> Hand {
        let parts: Vec<_> = hand.split(" ").collect();
        let cards: Vec<_> = parts[0].chars().collect();
        let bid = parts[1].parse::<usize>().unwrap();

        let card_occurrences: HashMap<_, _> = cards.iter()
            .sorted()
            .group_by(|&x| x)
            .into_iter()
            .map(|(c, v)| (c, v.count()))
            .collect();

        let hand_type = find_hand_type(card_occurrences);

        Hand { cards, bid, hand_type }
    }
}

fn find_hand_type_with_jolly(mut card_occurrences: HashMap<&char, usize>) -> HandType {
    if !card_occurrences.contains_key(&'J') {
        return find_hand_type(card_occurrences);
    }

    let jolly_occurrences = card_occurrences.remove(&'J').unwrap();
    let occurrences: HashMap<_, _> = score_occurrences(card_occurrences);

    if occurrences.contains_key(&4) {
        return FiveOfAKind;
    }

    if occurrences.contains_key(&3) && jolly_occurrences == 2 {
        return FiveOfAKind;
    }

    if occurrences.contains_key(&3) {
        return FourOfAKind;
    }

    if occurrences.contains_key(&2) && *occurrences.get(&2).unwrap() == 2 {
        return FullHouse;
    }

    if occurrences.contains_key(&2) && jolly_occurrences == 3 {
        return FiveOfAKind;
    }

    if occurrences.contains_key(&2) && jolly_occurrences == 2 {
        return FourOfAKind;
    }

    if occurrences.contains_key(&2) && jolly_occurrences == 1 {
        return ThreeOfAKind;
    }

    match jolly_occurrences {
        1 => OnePair,
        2 => ThreeOfAKind,
        3 => FourOfAKind,
        _ => FiveOfAKind
    }
}

fn find_hand_type(card_occurrences: HashMap<&char, usize>) -> HandType {
    let occurrences = score_occurrences(card_occurrences);

    if occurrences.contains_key(&5) {
        return FiveOfAKind;
    }

    if occurrences.contains_key(&4) {
        return FourOfAKind;
    }

    if occurrences.contains_key(&3) && occurrences.contains_key(&2) {
        return FullHouse;
    }

    if occurrences.contains_key(&3) {
        return ThreeOfAKind;
    }

    if occurrences.contains_key(&2) && *occurrences.get(&2).unwrap() == 2 {
        return TwoPairs;
    }

    if occurrences.contains_key(&2) {
        return OnePair;
    }

    HighCard
}

fn score_occurrences(card_occurrences: HashMap<&char, usize>) -> HashMap<usize, usize> {
    card_occurrences
        .iter()
        .map(|(_, v)| v)
        .sorted()
        .group_by(|&x| x)
        .into_iter()
        .map(|(k, v)| (k.clone(), v.count()))
        .collect()
}

const CARD_ORDER: [char; 13] = ['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A'];
const CARD_ORDER_WITH_JOLLY: [char; 13] = ['J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A'];

fn hand_comparator(current: &Hand, other: &Hand, card_order: [char; 13]) -> Ordering {
    if current.hand_type != other.hand_type {
        return current.hand_type.cmp(&other.hand_type);
    }

    for (&c1, &c2) in current.cards.iter().zip(other.cards.iter()) {
        if c1 != c2 {
            let c1_order = card_order.iter().position(|&r| r == c1).unwrap();
            let c2_order = card_order.iter().position(|&r| r == c2).unwrap();
            return c1_order.cmp(&c2_order);
        }
    }

    return Ordering::Equal;
}

fn total_winning(input: &str, card_order: [char; 13], hand_builder: fn(&String) -> Hand) -> usize {
    read_lines(input).iter()
        .map(hand_builder)
        .sorted_by(|current, other| hand_comparator(current, other, card_order))
        .enumerate()
        .map(|(rank, h)| h.bid * (rank + 1))
        .sum()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::day7::*;
    use crate::input_reader::read_input_file;

    #[test]
    fn it_solves_first_part() {
        let input = read_input_file("input_day07.txt");

        assert_eq!(251287184, total_winning(&input, CARD_ORDER, |h| Hand::new(h)));
    }

    #[test]
    fn it_solves_second_part() {
        let input = read_input_file("input_day07.txt");

        assert_eq!(250757288, total_winning(&input, CARD_ORDER_WITH_JOLLY, |h| Hand::build_with(h, find_hand_type_with_jolly)));
    }

    #[test]
    fn it_calculates_total_winning_with_jolly() {
        let input = indoc! {"
        32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483"};

        assert_eq!(5905, total_winning(input, CARD_ORDER_WITH_JOLLY, |h| Hand::build_with(h, find_hand_type_with_jolly)));
    }

    #[test]
    fn it_calculates_total_winning() {
        let input = indoc! {"
        32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483"};

        assert_eq!(6440, total_winning(input, CARD_ORDER, |h| Hand::new(h)));
    }

    #[test]
    fn it_parses_a_hand_with_jolly() {
        assert_eq!(Hand { cards: vec!['3', '2', 'T', '3', 'K'], bid: 765, hand_type: OnePair }, Hand::build_with("32T3K 765", find_hand_type_with_jolly));
        assert_eq!(Hand { cards: vec!['T', '5', '5', 'J', '5'], bid: 684, hand_type: FourOfAKind }, Hand::build_with("T55J5 684", find_hand_type_with_jolly));
        assert_eq!(Hand { cards: vec!['K', 'K', '6', '7', '7'], bid: 28, hand_type: TwoPairs }, Hand::build_with("KK677 28", find_hand_type_with_jolly));
        assert_eq!(Hand { cards: vec!['K', 'T', 'J', 'J', 'T'], bid: 220, hand_type: FourOfAKind }, Hand::build_with("KTJJT 220", find_hand_type_with_jolly));
        assert_eq!(Hand { cards: vec!['Q', 'Q', 'Q', 'J', 'A'], bid: 483, hand_type: FourOfAKind }, Hand::build_with("QQQJA 483", find_hand_type_with_jolly));
    }

    #[test]
    fn it_compares_hand_type() {
        assert!(TwoPairs > OnePair);
        assert!(FullHouse > TwoPairs);
    }

    #[test]
    fn it_parses_a_hand() {
        assert_eq!(Hand { cards: vec!['3', '2', 'T', '3', 'K'], bid: 765, hand_type: OnePair }, Hand::new("32T3K 765"));
        assert_eq!(Hand { cards: vec!['T', '5', '5', 'J', '5'], bid: 684, hand_type: ThreeOfAKind }, Hand::new("T55J5 684"));
        assert_eq!(Hand { cards: vec!['K', 'K', '6', '7', '7'], bid: 28, hand_type: TwoPairs }, Hand::new("KK677 28"));
        assert_eq!(Hand { cards: vec!['K', 'T', 'J', 'J', 'T'], bid: 220, hand_type: TwoPairs }, Hand::new("KTJJT 220"));
        assert_eq!(Hand { cards: vec!['Q', 'Q', 'Q', 'J', 'A'], bid: 483, hand_type: ThreeOfAKind }, Hand::new("QQQJA 483"));
    }
}