use std::collections::HashSet;

use regex::Regex;
use crate::input_reader::read_lines;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Card {
    index: usize,
    winning_numbers: HashSet<usize>,
    numbers: HashSet<usize>
}

impl Card {
    fn build_from(raw_card: &str) -> Self {
        let parts: Vec<_> = raw_card.split(": ").collect();
        let card_with_index = parts[0];
        let numbers: Vec<_> = parts[1].split(" | ").collect();
        let index = parse_card_index(card_with_index);
        let winning_numbers: HashSet<_> = parse_numbers(numbers[0]);
        let numbers: HashSet<_> = parse_numbers(numbers[1]);
        Card { index, winning_numbers, numbers }
    }

    fn score(&self) -> usize {
        let overlapping_cards = self.numbers.intersection(&self.winning_numbers).count();
        if overlapping_cards == 0 {
            return 0;
        }

        1 << (overlapping_cards - 1)
    }
}

fn parse_card_index(card_with_index: &str) -> usize {
    Regex::new(r"(\d+)").unwrap()
        .captures(card_with_index)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .parse::<usize>()
        .unwrap()
}

fn parse_numbers(values: &str) -> HashSet<usize> {
    Regex::new(r"(\d+)")
        .unwrap()
        .captures_iter(values)
        .map(|c| c.extract())
        .map(|(_, [value])| value.parse::<usize>().unwrap())
        .collect()
}

fn deck_score(input: &str) -> usize {
    read_lines(input)
        .iter()
        .map(|c| Card::build_from(c).score())
        .sum()
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use indoc::indoc;
    use crate::day4::*;
    use crate::input_reader::read_input_file;

    #[test]
    fn it_solves_first_part() {
        let input = read_input_file("input_day04.txt");

        assert_eq!(24706, deck_score(&input));
    }

    #[test]
    fn it_calculates_deck_score() {
        let input = indoc! {"
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"};

        assert_eq!(13, deck_score(input));
    }

    #[test]
    fn it_calculates_card_score() {
        assert_eq!(8, Card::build_from("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53").score());
        assert_eq!(2, Card::build_from("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19").score());
        assert_eq!(1, Card::build_from("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83").score());
        assert_eq!(0, Card::build_from("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36").score());
    }

    #[test]
    fn it_parses_a_card() {
        let card = Card::build_from("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
        assert_eq!(1, card.index);
        assert_eq!(HashSet::from([48, 41, 86, 83, 17]), card.winning_numbers);
        assert_eq!(HashSet::from([83,86,6,31,17,9,48,53]), card.numbers);
    }
}