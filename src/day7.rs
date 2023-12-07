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

#[derive(Debug, Clone, Eq, PartialEq, Ord)]
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

fn find_hand_type(card_occurrences: HashMap<&char, usize>) -> HandType {
    let occurrences: HashMap<_, _> = card_occurrences
        .iter()
        .map(|(_, v)| v)
        .sorted()
        .group_by(|&x| x)
        .into_iter()
        .map(|(k, v)| (k, v.count()))
        .collect();

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

const CARD_ORDER: [char; 13] = ['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A'];

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.hand_type != other.hand_type {
            return self.hand_type.partial_cmp(&other.hand_type);
        }

        for (&c1, &c2) in self.cards.iter().zip(other.cards.iter()) {
            if c1 != c2 {
                let c1_order = CARD_ORDER.iter().position(|&r| r == c1).unwrap();
                let c2_order = CARD_ORDER.iter().position(|&r| r == c2).unwrap();
                return c1_order.partial_cmp(&c2_order);
            }
        }

        return Some(Ordering::Equal);
    }
}

fn total_winning(input: &str) -> usize {
    let sorted_hands: Vec<_> = read_lines(input).iter()
        .map(|h| Hand::new(h))
        .sorted()
        .enumerate()
        .collect();

    sorted_hands.iter()
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

        assert_eq!(251287184, total_winning(&input));
    }

    #[test]
    fn it_calculates_total_winning() {
        let input = indoc! {"
        32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483"};

        assert_eq!(6440, total_winning(input));
    }

    #[test]
    fn it_compares_hands() {
        assert!(Hand::new("KK677 28") > Hand::new("32T3K 765"));
        assert!(Hand::new("QQQJA 28") > Hand::new("T55J5 765"));
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