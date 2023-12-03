use std::collections::HashMap;
use std::fmt;
use std::fmt::Display;

use crate::input_reader::read_lines;

struct EngineSchematic {
    height: usize,
    width: usize,
    numbers: Vec<Number>,
    schema: Vec<String>,
}

impl EngineSchematic {
    fn build_form(input: &str) -> EngineSchematic {
        let schema = read_lines(input);

        let height = schema.len();
        let width = schema.first().unwrap().len();

        let numbers: Vec<_> = schema.iter()
            .enumerate()
            .flat_map(|(row_index, line)| find_numbers_in_line(row_index, line))
            .collect();

        EngineSchematic { height, width, numbers, schema }
    }

    fn find_numbers_close_to_symbols(&self) -> Vec<Number> {
        let mut numbers_close_to_symbols = vec![];

        for number in &self.numbers {
            let any_symbol = number.neighbours_positions()
                .iter()
                .filter(|(x, y)| *x < self.width && *y < self.height)
                .any(|(x, y)| -> bool  {
                    let neighbour = self.schema[*y].chars().nth(*x).unwrap();
                    neighbour != '.' && !neighbour.is_ascii_digit()
                });

            if any_symbol {
                numbers_close_to_symbols.push(number.clone());
            }
        }

        numbers_close_to_symbols
    }

    fn sum_numbers_close_to_symbols(&self) -> usize {
        self.find_numbers_close_to_symbols().iter()
            .map(|n| n.value)
            .sum()
    }

    fn find_multiplier_operations(&self) -> HashMap<(usize, usize), Vec<Number>> {
        let mut result = HashMap::new();

        for number in &self.numbers {
            number
                .neighbours_positions()
                .iter()
                .filter(|(x, y)| *x < self.width && *y < self.height)
                .filter_map(|(x, y)| {
                    let value = self.schema.get(*y)?.chars().nth(*x)?;
                    if value == '*' {
                        Some(((*x, *y), value))
                    } else {
                        None
                    }
                })
                .for_each(|(position, _)| {
                    let entry = result.entry(position.clone()).or_insert(Vec::new());
                    entry.push(number.clone());
                });
        }

        result
    }
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
struct Number {
    row_index: usize,
    value: usize,
    start: usize,
    end: usize,
}

impl Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "row_index: {}, value: {}, start: {}, end: {})", self.row_index, self.value, self.start, self.end)
    }
}

impl Number {
    fn new(row_index: usize, start: usize, value: &str) -> Self {
        let end = start + value.len() - 1;
        let value = value.parse::<usize>().unwrap();
        Number { row_index, start, end, value }
    }

    fn neighbours_positions(&self) -> Vec<(usize, usize)> {
        let mut neighbours_positions = vec![];

        let mut start = self.start;
        if self.start != 0 {
            start -= 1;
            neighbours_positions.push((self.start - 1, self.row_index));
        }

        neighbours_positions.push((self.end + 1, self.row_index));
        for x in start..=(self.end + 1) {
            if self.row_index != 0 {
                neighbours_positions.push((x, self.row_index - 1));
            }
            neighbours_positions.push((x, self.row_index + 1));
        }

        neighbours_positions
    }
}

fn find_numbers_in_line(row_index: usize, row: &str) -> Vec<Number> {
    let mut numbers_in_line = vec![];
    let mut current_index = 0;
    let chars: Vec<char> = row.chars().collect();
    while current_index < row.len() {
        if !chars[current_index].is_numeric() {
            current_index += 1;
            continue;
        }

        let mut number: Vec<char> = vec![];
        let start = current_index;
        while current_index < row.len() && chars[current_index].is_numeric() {
            number.push(chars[current_index]);
            current_index += 1;
        }
        let value: String = number.iter().collect();
        numbers_in_line.push(Number::new(row_index, start, &value));
    }
    numbers_in_line
}

fn sum_numbers_close_to_symbols(input: &str) -> usize {
    EngineSchematic::build_form(input).sum_numbers_close_to_symbols()
}

fn sum_gear_ratios(input: &str) -> usize {
    EngineSchematic::build_form(input)
        .find_multiplier_operations().iter()
        .filter_map(|(_, numbers)| {
            if numbers.len() == 2 {
                Some(numbers.iter().map(|n| n.value).fold(1, |acc, x| acc * x))
            } else {
                None
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::day3::*;
    use crate::input_reader::read_input_file;

    #[test]
    fn it_solves_first_part() {
        let input = read_input_file("input_day03.txt");

        assert_eq!(560670, sum_numbers_close_to_symbols(&input));
    }

    #[test]
    fn it_solves_second_part() {
        let input = read_input_file("input_day03.txt");

        assert_eq!(91622824, sum_gear_ratios(&input));
    }

    #[test]
    fn it_finds_multiplier_operations() {
        let input = indoc! {"
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598.."};

        assert_eq!(467835, sum_gear_ratios(input));
    }

    #[test]
    fn it_sums_numbers_close_to_symbols() {
        let input = indoc! {"
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598.."};

        assert_eq!(4361, sum_numbers_close_to_symbols(input));
    }

    #[test]
    fn it_finds_neighbours_positions() {
        assert_eq!(vec![(3, 0), (0, 1), (1, 1), (2, 1), (3, 1)],
                   Number {
                       row_index: 0,
                       value: 467,
                       start: 0,
                       end: 2,
                   }.neighbours_positions());

        assert_eq!(vec![(1, 2), (4, 2),
                        (1, 1), (1, 3),
                        (2, 1), (2, 3),
                        (3, 1), (3, 3),
                        (4, 1), (4, 3)],
                   Number {
                       row_index: 2,
                       value: 35,
                       start: 2,
                       end: 3,
                   }.neighbours_positions());
    }

    #[test]
    fn it_finds_numbers_in_line() {
        assert_eq!(vec![
            Number {
                row_index: 0,
                value: 467,
                start: 0,
                end: 2,
            },
            Number {
                row_index: 0,
                value: 114,
                start: 5,
                end: 7,
            }], find_numbers_in_line(0, "467..114.."));

        assert_eq!(vec![
            Number {
                row_index: 0,
                value: 467,
                start: 0,
                end: 2,
            },
            Number {
                row_index: 0,
                value: 467,
                start: 5,
                end: 7,
            }], find_numbers_in_line(0, "467..467.."));

        assert_eq!(vec![
            Number {
                row_index: 0,
                value: 122,
                start: 0,
                end: 2,
            },
            Number {
                row_index: 0,
                value: 22,
                start: 5,
                end: 6,
            }], find_numbers_in_line(0, "122..22..."));
    }
}