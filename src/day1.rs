use regex::Regex;

use crate::input_reader::read_lines;

fn find_numbers(line: &str, re_number: Regex) ->  Vec<&str> {
    re_number.find_iter(line).map(|m| m.as_str()).collect()
}

fn find_numbers2(line: &str) ->  Vec<&str> {
    let mut numbers: Vec<(usize, &str)> = vec![];

    for n in 1..=9 {
        numbers.extend(line.match_indices(&n.to_string()));
    }

    for n in ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"] {
        numbers.extend(line.match_indices(&n.to_string()));
    }

    numbers.sort();

    numbers.iter().map(|(_, n)| *n).collect()
}

fn sum_calibration_values(input: &str) -> u64 {
    let lines = read_lines(input);
    lines.iter().map(|line| -> u64 {
        let numbers = find_numbers(line, Regex::new(r"\d").unwrap());
        format!("{}{}", numbers.first().unwrap(), numbers.last().unwrap()).parse::<u64>().unwrap()
    }
    ).sum()
}

fn sum_calibration_values_extended(input: &str) -> u64 {
    let lines = read_lines(input);
    lines.iter().map(|line| -> u64 {
        let numbers = find_numbers2(line);
        format!("{}{}", convert_number(numbers.first().unwrap()), convert_number(numbers.last().unwrap())).parse::<u64>().unwrap()
    }
    ).sum()
}

fn convert_number(number: &str) -> u64 {
    let numbers = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    match number.parse::<u64>() {
        Ok(n) => n,
        _ => numbers.iter().position(|&r| r == number).unwrap() as u64 + 1
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::day1::*;
    use crate::input_reader::read_input_file;

    #[test]
    fn converts_a_number() {
        assert_eq!(1, convert_number("1"));
        assert_eq!(1, convert_number("one"));
    }

    #[test]
    fn it_finds_all_numbers() {
        let line = "1abc2";
        assert_eq!(vec!["1", "2"], find_numbers(line, Regex::new(r"\d").unwrap()));

        let line = "7pqrstsixteen";
        assert_eq!(vec!["7", "six"], find_numbers(line, Regex::new(r"(one|two|three|four|five|six|seven|eight|nine|\d)").unwrap()));


        let line = "4nineeightseven2";
        assert_eq!(vec!["4", "nine", "eight", "seven", "2"], find_numbers(line, Regex::new(r"(one|two|three|four|five|six|seven|eight|nine|\d)").unwrap()));

        let line = "85dsixsevenmpcclxdjvsvppkpflhxqvgsjnbsvlcgv";
        assert_eq!(vec!["8", "5", "six", "seven"], find_numbers(line, Regex::new(r"(one|two|three|four|five|six|seven|eight|nine|\d)").unwrap()));

        let line = "four5twone7";
        // assert_eq!(vec![(0, "two")], line.match_indices("two").collect::<Vec<_>>());
        // assert_eq!(vec![(2, "one")], line.match_indices("one").collect::<Vec<_>>());
        // assert_eq!(vec![(5, "7")], line.match_indices('7').collect::<Vec<_>>());
        let mut numbers: Vec<(usize, &str)> = vec![];

        for n in 1..=9 {
            numbers.extend(line.match_indices(&n.to_string()));
        }

        for n in ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"] {
            numbers.extend(line.match_indices(&n.to_string()));
        }

        numbers.sort();
        assert_eq!(vec![(0, "four"), (4, "5"), (5, "two"), (7, "one"), (10, "7")], numbers);

        let sorted_numbers: Vec<& str> = numbers.iter().map(|(_, n)| *n).collect();
        assert_eq!(vec!["four", "5", "two", "one", "7"], sorted_numbers);
    }

    #[test]
    fn it_finds_lines_sum() {
        let input = indoc! {"
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet"};

        let sum = sum_calibration_values(input);

        assert_eq!(142, sum);
    }

    #[test]
    fn it_solves_first_puzzle_part_1() {
        let input = read_input_file("input_day01.txt");

        let sum = sum_calibration_values(&input);

        assert_eq!(55090, sum);
    }

    #[test]
    fn it_finds_lines_sum_extended() {
        let input = indoc! {"
        two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen"};

        let sum = sum_calibration_values_extended(input);

        assert_eq!(281, sum);
    }

    #[test]
    fn it_solves_first_puzzle_part_2() {
        let input = read_input_file("input_day01.txt");

        let sum = sum_calibration_values_extended(&input);

        assert_eq!(54845, sum);
    }
}