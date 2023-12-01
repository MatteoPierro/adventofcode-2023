use crate::input_reader::read_lines;

const NUMBERS_AS_WORDS: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn sum_calibration_values(input: &str, extra_matches: &Vec<&str>) -> usize {
    read_lines(input).iter()
        .map(|line| line_number(line, extra_matches))
        .sum()
}

fn line_number(line: &str, extra_matches: &Vec<&str>) -> usize {
    let numbers = find_numbers(line, extra_matches);
    format!("{}{}", numbers.first().unwrap(), numbers.last().unwrap())
        .parse::<usize>()
        .unwrap()
}

fn find_numbers(line: &str, extra_matches: &Vec<&str>) -> Vec<usize> {
    let mut numbers: Vec<_> = line.match_indices(char::is_numeric).collect();

    for n in extra_matches {
        numbers.extend(line.match_indices(n));
    }

    numbers.sort();

    numbers.iter()
        .map(|(_, n)| convert_number(n))
        .collect()
}

fn convert_number(number: &str) -> usize {
    match number.parse::<usize>() {
        Ok(n) => n,
        _ => NUMBERS_AS_WORDS.iter().position(|&r| r == number).unwrap() + 1
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::day1::*;
    use crate::input_reader::read_input_file;

    #[test]
    fn it_finds_lines_sum() {
        let input = indoc! {"
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet"};

        assert_eq!(142, sum_calibration_values(input, &vec![]));
    }

    #[test]
    fn it_solves_puzzle_part_1() {
        let input = read_input_file("input_day01.txt");

        assert_eq!(55090, sum_calibration_values(&input, &vec![]));
    }

    #[test]
    fn it_finds_lines_sum_with_numbers_as_words() {
        let input = indoc! {"
        two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen"};

        assert_eq!(281, sum_calibration_values(input, &NUMBERS_AS_WORDS.to_vec()));
    }

    #[test]
    fn it_solves_puzzle_part_2() {
        let input = read_input_file("input_day01.txt");

        assert_eq!(54845, sum_calibration_values(&input, &NUMBERS_AS_WORDS.to_vec()));
    }
}