use std::collections::HashSet;

use itertools::Itertools;

use crate::input_reader::read_lines;

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
struct Position(usize, usize);

fn calculate_total_load(input: &str) -> usize {
    let mut rounded_rocks: HashSet<Position> = HashSet::new();
    let mut cube_shaped_rocks: HashSet<Position> = HashSet::new();
    let lines = read_lines(input);
    for (y, row) in lines.iter().enumerate() {
        for (x, c) in row.chars().enumerate() {
            if c == 'O' {
                rounded_rocks.insert(Position(x, y));
            }
            if c == '#' {
                cube_shaped_rocks.insert(Position(x, y));
            }
        }
    }

    // println!("{:?}", rounded_rocks);
    // println!("{:?}", cube_shaped_rocks);

    loop {
        let mut next_rounded_rocks: HashSet<Position> = HashSet::new();
        for Position(x, y) in &rounded_rocks {
            if *y == 0 {
                next_rounded_rocks.insert(Position(*x, *y));
                continue;
            }

            let next_position = Position(*x, y - 1);
            if rounded_rocks.contains(&next_position) || cube_shaped_rocks.contains(&next_position) {
                next_rounded_rocks.insert(Position(*x, *y));
            } else {
                next_rounded_rocks.insert(next_position);
            }
        }

        if next_rounded_rocks == rounded_rocks {
            break;
        }

        rounded_rocks = next_rounded_rocks;
    }

    let rounded_rocks_by_y = rounded_rocks.iter().into_group_map_by(|p1| p1.1);

    let dish_len = lines.len();

    let mut total_load: usize = 0;
    for y in 0..dish_len {
        if let Some(rocks) = rounded_rocks_by_y.get(&y) {
            total_load += (dish_len - y) * rocks.len();
        }
    }
    total_load
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::day14::*;
    use crate::input_reader::read_input_file;

    #[test]
    fn it_solves_first_puzzle() {
        let input = &read_input_file("input_day14.txt");

        assert_eq!(108826, calculate_total_load(input));
    }

    #[test]
    fn it_calculates_total_load() {
        let input = indoc! {"
        O....#....
        O.OO#....#
        .....##...
        OO.#O....O
        .O.....O#.
        O.#..O.#.#
        ..O..#O..O
        .......O..
        #....###..
        #OO..#...."};

        assert_eq!(136, calculate_total_load(input));
    }
}