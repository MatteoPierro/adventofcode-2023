use std::ops::Div;

use crate::input_reader::read_lines;

fn calculate_cubic_meters_of_lava(input: &str) -> usize {
    let mut digger = Digger::new();
    let tranches: Vec<Trench> = digger.find_tranches(input);
    tranches.len() + number_of_internal_tranches(&tranches)
}

// Calculate enclosed point using Pick's theorem
// https://en.wikipedia.org/wiki/Pick's_theorem
fn number_of_internal_tranches(tranches: &Vec<Trench>) -> usize {
    let polygon = tranches.iter().map(|t| t.position).collect::<Vec<_>>();
    let area = calculate_area(&polygon);
    let b = polygon.iter().count() as isize;
    (area + 1 - (b / 2)) as usize
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

struct Trench {
    position: Position,
    color: String,
}

struct Digger {
    current_position: Position,
}

impl Digger {
    fn new() -> Self {
        Digger { current_position: Position(0, 0) }
    }
    fn find_tranches(&mut self, input: &str) -> Vec<Trench> {
        let mut trenches: Vec<Trench> = vec![];

        for line in read_lines(input) {
            let line_parts = line.split(" (").collect::<Vec<_>>();
            let dig_instruction = line_parts[0];
            let color = line_parts[1].split(")").collect::<Vec<_>>()[0];
            let instruction_parts = dig_instruction.split(" ").collect::<Vec<_>>();
            let direction = instruction_parts[0];
            let steps = instruction_parts[1].parse::<usize>().unwrap();
            self.dig(&mut trenches, direction, steps, color)
        }

        trenches
    }

    fn dig(&mut self, positions: &mut Vec<Trench>, direction: &str, steps: usize, color: &str) {
        match direction {
            "U" => self.dig_up(positions, steps, color),
            "D" => self.dig_down(positions, steps, color),
            "R" => self.dig_right(positions, steps, color),
            "L" => self.dig_left(positions, steps, color),
            _ => panic!("unknown instruction")
        }
    }

    fn dig_up(&mut self, positions: &mut Vec<Trench>, steps: usize, color: &str) {
        for _ in 1..=steps {
            let next_position = Position(self.current_position.0, self.current_position.1 - 1);
            positions.push(Trench { position: next_position, color: color.to_string() });
            self.current_position = next_position.clone();
        }
    }

    fn dig_down(&mut self, positions: &mut Vec<Trench>, steps: usize, color: &str) {
        for _ in 1..=steps {
            let next_position = Position(self.current_position.0, self.current_position.1 + 1);
            positions.push(Trench { position: next_position, color: color.to_string() });
            self.current_position = next_position.clone();
        }
    }

    fn dig_right(&mut self, positions: &mut Vec<Trench>, steps: usize, color: &str) {
        for _ in 1..=steps {
            let next_position = Position(self.current_position.0 + 1, self.current_position.1);
            positions.push(Trench { position: next_position, color: color.to_string() });
            self.current_position = next_position.clone();
        }
    }

    fn dig_left(&mut self, positions: &mut Vec<Trench>, steps: usize, color: &str) {
        for _ in 1..=steps {
            let next_position = Position(self.current_position.0 - 1, self.current_position.1);
            positions.push(Trench { position: next_position, color: color.to_string() });
            self.current_position = next_position.clone();
        }
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::day18::*;
    use crate::input_reader::read_input_file;

    #[test]
    fn it_solves_first_part() {
        let input = &read_input_file("input_day18.txt");

        assert_eq!(45159, calculate_cubic_meters_of_lava(input));
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

        assert_eq!(62, calculate_cubic_meters_of_lava(input));
    }
}