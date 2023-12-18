use std::ops::Div;

use crate::input_reader::read_lines;

fn calculate_cubic_meters_of_lava(input: &str, digger: &mut Digger) -> isize {
    digger.find_tranches(input);
    digger.perimeter + number_of_internal_tranches(&digger.polygon, digger.perimeter)
}

// Calculate number of internal tranches using Pick's theorem
// https://en.wikipedia.org/wiki/Pick's_theorem
fn number_of_internal_tranches(polygon: &Vec<Position>, perimeter: isize) -> isize {
    let area = calculate_area(&polygon);
    let b = perimeter;
    area + 1 - (b / 2)
}

// Calculate area using the Shoelace formula
// https://en.wikipedia.org/wiki/Shoelace_formula
fn calculate_area(polygon: &Vec<Position>) -> isize {
    polygon.windows(2)
        .map(|positions| {
            let Position(x1, y1) = positions[0];
            let Position(x2, y2) = positions[1];
            (x1 * y2) as isize - (x2 * y1) as isize
        })
        .sum::<isize>()
        .abs()
        .div(2)
}

#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
struct Position(isize, isize);

struct Digger {
    current_position: Position,
    perimeter: isize,
    polygon: Vec<Position>,
    instruction_parser: fn(String) -> (String, isize)
}

impl Digger {
    fn new() -> Self {
        Digger {
            current_position: Position(0, 0),
            perimeter: 0,
            polygon: vec![Position(0, 0)],
            instruction_parser: parse_instruction
        }
    }

    fn build_with_color_instruction() -> Self {
        Digger {
            current_position: Position(0, 0),
            perimeter: 0,
            polygon: vec![Position(0, 0)],
            instruction_parser: parse_instruction_color
        }
    }

    fn find_tranches(&mut self, input: &str) {
        for line in read_lines(input) {
            let(direction, steps) = (self.instruction_parser)(line);
            self.perimeter += steps;
            let next_position = self.dig(&direction, steps);
            self.current_position = next_position.clone();
            self.polygon.push(self.current_position.clone());
        }
    }

    fn dig(&mut self, direction: &str, steps: isize) -> Position {
        match direction {
            "U" => Position(self.current_position.0, self.current_position.1 - steps),
            "D" => Position(self.current_position.0, self.current_position.1 + steps),
            "R" => Position(self.current_position.0 + steps, self.current_position.1),
            "L" => Position(self.current_position.0 - steps, self.current_position.1),
            _ => panic!("unknown instruction")
        }
    }
}

fn parse_instruction(raw_instruction: String) -> (String, isize) {
    let line_parts = raw_instruction.split(" (").collect::<Vec<_>>();
    let dig_instruction = line_parts[0];
    let instruction_parts = dig_instruction.split(" ").collect::<Vec<_>>();
    let direction = instruction_parts[0];
    let steps = instruction_parts[1].parse::<isize>().unwrap();
    (direction.to_string(), steps)
}

fn parse_instruction_color(raw_instruction: String) -> (String, isize) {
    let line_parts = raw_instruction.split(" (").collect::<Vec<_>>();
    let color = line_parts[1].split(")").collect::<Vec<_>>()[0].chars().collect::<Vec<_>>();
    let direction = match color.last().unwrap() {
        '0' => "R",
        '1' => "D",
        '2' => "L",
        '3' => "U",
        _ => panic!("unexpected direction")
    };
    let hex = color[1..color.len() - 1].iter().collect::<String>();
    let steps = isize::from_str_radix(&hex, 16).unwrap();
    (direction.to_string(), steps)
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::day18::*;
    use crate::input_reader::read_input_file;

    #[test]
    fn it_solves_puzzle() {
        let input = &read_input_file("input_day18.txt");

        assert_eq!(45159, calculate_cubic_meters_of_lava(input, &mut Digger::new()));
        assert_eq!(134549294799713, calculate_cubic_meters_of_lava(input, &mut Digger::build_with_color_instruction()));
    }

    #[test]
    fn it_calculates_cubic_meters_of_lava() {
        let input = indoc! {"
        R 6 (#70c710)
        D 5 (#0dc571)
        L 2 (#5713f0)
        D 2 (#d2c081)
        R 2 (#59c680)
        D 2 (#411b91)
        L 5 (#8ceee2)
        U 2 (#caa173)
        L 1 (#1b58a2)
        U 2 (#caa171)
        R 2 (#7807d2)
        U 3 (#a77fa3)
        L 2 (#015232)
        U 2 (#7a21e3)"};

        assert_eq!(62, calculate_cubic_meters_of_lava(input, &mut Digger::new()));
        assert_eq!(952408144115, calculate_cubic_meters_of_lava(input, &mut Digger::build_with_color_instruction()));
    }

    #[test]
    fn it_parses_color_instruction() {
        assert_eq!(("R".to_string(), 461937), parse_instruction_color("R 6 (#70c710)".to_string()))
    }
}