use num::Zero;

use crate::input_reader::read_lines;

fn find_next_values(mut sequence: Vec<isize>) -> (isize, isize) {
    let mut first_numbers = vec![sequence.first().unwrap().clone()];
    let mut last_numbers = vec![sequence.last().unwrap().clone()];

    loop {
        let new_sequence: Vec<_> = sequence.windows(2)
            .map(|elements| elements[1] - elements[0])
            .collect();

        if new_sequence.iter().all(|n| n.is_zero()) {
            break;
        }

        sequence = new_sequence;
        first_numbers.push(sequence.first().unwrap().clone());
        last_numbers.push(sequence.last().unwrap().clone());
    }

    (
        first_numbers.iter().rev().fold(0, |acc, e| e - acc),
        last_numbers.iter().sum()
    )
}

fn calculate_sum_of_next_values(input: &str) -> (isize, isize) {
    read_lines(input).iter()
        .map(|sequence| parse_sequence(sequence))
        .map(|sequence| find_next_values(sequence))
        .fold((0, 0), |(acc0, acc1), (e0, e1)| (acc0 + e0, acc1 + e1))
}

fn parse_sequence(sequence: &String) -> Vec<isize> {
    sequence.split(" ")
        .map(|n| n.parse::<isize>().unwrap())
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::day9::*;
    use crate::input_reader::read_input_file;

    #[test]
    fn it_solves_puzzle() {
        let input = read_input_file("input_day09.txt");

        assert_eq!((900, 1987402313), calculate_sum_of_next_values(&input));
    }

    #[test]
    fn it_calculates_sum_of_next_values() {
        let input = indoc! {"
        0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45"};

        assert_eq!((2, 114), calculate_sum_of_next_values(input));
    }

    #[test]
    fn it_finds_next_values() {
        assert_eq!((-3, 18), find_next_values(vec![0, 3, 6, 9, 12, 15]));
        assert_eq!((0, 28), find_next_values(vec![1, 3, 6, 10, 15, 21]));
        assert_eq!((5, 68), find_next_values(vec![10, 13, 16, 21, 30, 45]));
    }
}