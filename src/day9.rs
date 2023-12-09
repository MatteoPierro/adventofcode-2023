use num::Zero;

use crate::input_reader::read_lines;

fn find_next_value(mut sequence: Vec<isize>) -> isize {
    let mut last_numbers = vec![sequence.last().unwrap().clone()];

    loop {
        let new_sequence: Vec<_> = sequence.windows(2)
            .map(|elements| elements[1] - elements[0])
            .collect();

        if new_sequence.iter().all(|n| n.is_zero()) {
            break;
        }

        sequence = new_sequence;
        last_numbers.push(sequence.last().unwrap().clone());
    }

    last_numbers.iter().sum()
}

fn calculate_sum_of_next_values(input: &str) -> isize {
    read_lines(input).iter()
        .map(|sequence| parse_sequence(sequence))
        .map(|sequence| find_next_value(sequence))
        .sum()
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
    fn it_solves_first_part() {
        let input = read_input_file("input_day09.txt");

        assert_eq!(1987402313, calculate_sum_of_next_values(&input));
    }

    #[test]
    fn it_calculates_sum_of_next_values() {
        let input = indoc! {"
        0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45"};

        assert_eq!(114, calculate_sum_of_next_values(input));
    }

    #[test]
    fn it_finds_next_value() {
        assert_eq!(18, find_next_value(vec![0, 3, 6, 9, 12, 15]));
        assert_eq!(28, find_next_value(vec![1, 3, 6, 10, 15, 21]));
        assert_eq!(68, find_next_value(vec![10, 13, 16, 21, 30, 45]));
    }
}