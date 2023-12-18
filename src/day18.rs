use std::ops::Div;
use crate::input_reader::read_lines;

fn calculate_cubic_meters_of_lava(input: &str) -> usize {
    let mut digger = Digger { current_position: Position(0, 0) };
    let mut positions: Vec<Position> = vec![];

    for line in read_lines(input) {
        let line_parts = line.split(" (").collect::<Vec<_>>();
        let dig_instruction = line_parts[0];
        let instruction_parts = dig_instruction.split(" ").collect::<Vec<_>>();
        let direction = instruction_parts[0];
        let steps = instruction_parts[1].parse::<usize>().unwrap();
        digger.dig(&mut positions, direction, steps)
    }

    let area = calculate_area(&positions);
    let b = positions.iter().count() as isize;
    let internal_trench = (area + 1 - (b / 2)) as usize;

    positions.len() + internal_trench
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
    current_position: Position
}

impl Digger {
    fn dig(&mut self, positions: &mut Vec<Position>, direction: &str, steps: usize) {
        match direction {
            "U" => self.dig_up(positions, steps),
            "D" => self.dig_down(positions, steps),
            "R" => self.dig_right(positions, steps),
            "L" => self.dig_left(positions, steps),
            _ => panic!("unknown instruction")
        }
    }
    fn dig_up(&mut self, positions: &mut Vec<Position>, steps: usize) {
        for _ in 1..=steps {
            let next_position = Position(self.current_position.0, self.current_position.1 - 1);
            positions.push(next_position);
            self.current_position = next_position.clone();
        }
    }

    fn dig_down(&mut self, positions: &mut Vec<Position>, steps: usize) {
        for _ in 1..=steps {
            let next_position = Position(self.current_position.0, self.current_position.1 + 1);
            positions.push(next_position);
            self.current_position = next_position.clone();
        }
    }

    fn dig_right(&mut self, positions: &mut Vec<Position>, steps: usize) {
        for _ in 1..=steps {
            let next_position = Position(self.current_position.0 + 1, self.current_position.1);
            positions.push(next_position);
            self.current_position = next_position.clone();
        }
    }

    fn dig_left(&mut self, positions: &mut Vec<Position>, steps: usize) {
        for _ in 1..=steps {
            let next_position = Position(self.current_position.0 - 1, self.current_position.1);
            positions.push(next_position);
            self.current_position = next_position.clone();
        }
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use crate::day18::calculate_cubic_meters_of_lava;
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