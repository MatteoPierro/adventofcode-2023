use regex::Regex;
use crate::input_reader::read_lines;

fn find_numbers(line: &str) ->  Vec<&str> {
    let re_number = Regex::new(r"\d").unwrap();
    return re_number.find_iter(line).map(|m| m.as_str()).collect();
}

fn sum_calibration_values(input: &str) -> u64 {
    let lines = read_lines(input);
    lines.iter().map(|line| -> u64 {
        let numbers = find_numbers(line);
        format!("{}{}", numbers.first().unwrap(), numbers.last().unwrap()).parse::<u64>().unwrap()
    }
    ).sum()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use crate::day1::*;
    use crate::input_reader::read_input_file;

    #[test]
    fn it_finds_all_numbers() {
        let line = "1abc2";
        assert_eq!(vec!["1", "2"], find_numbers(line))
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
    fn it_solves_first_puzzle() {
        let input = read_input_file("input_day01.txt");

        let sum = sum_calibration_values(&input);

        assert_eq!(55090, sum);
    }
}