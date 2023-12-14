use std::collections::HashSet;

use itertools::Itertools;

use crate::input_reader::read_lines;

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
struct Position(usize, usize);

fn calculate_total_load(input: &str) -> usize {
    let (mut rounded_rocks, mut cube_shaped_rocks, lines) = parse_rocks(read_lines(input));

    let rounded_rocks = move_rock_north(&mut rounded_rocks, &mut cube_shaped_rocks);

    let dish_len = lines.len();

    total_load(&rounded_rocks, dish_len)
}

fn total_load(rounded_rocks: &HashSet<Position>, dish_len: usize) -> usize {
    let rounded_rocks_by_y = rounded_rocks.iter().into_group_map_by(|p1| p1.1);
    let mut total_load: usize = 0;
    for y in 0..dish_len {
        if let Some(rocks) = rounded_rocks_by_y.get(&y) {
            total_load += (dish_len - y) * rocks.len();
        }
    }
    total_load
}

fn calculate_total_load_with_cycle(input: &str) -> usize {
    let (mut rounded_rocks, mut cube_shaped_rocks, lines) = parse_rocks(read_lines(input));
    let number_of_columns = lines.len();
    let number_of_rows = lines[0].len();

    let mut periods: Vec<HashSet<Position>> = vec![];

    let mut cycle: usize = 0;
    let instant: usize;

    loop {
        rounded_rocks = move_rock_north(&mut rounded_rocks, &mut cube_shaped_rocks);
        rounded_rocks = move_rock_west(&mut rounded_rocks, &mut cube_shaped_rocks);
        rounded_rocks = move_rock_south(&mut rounded_rocks, &mut cube_shaped_rocks, number_of_columns);
        rounded_rocks = move_rock_east(&mut rounded_rocks, &mut cube_shaped_rocks, number_of_rows);
        cycle += 1;
        if let Some((i, _)) = periods.iter().find_position(|&p| *p == rounded_rocks) {
            instant = i + 1;
            break;
        }
        periods.push(rounded_rocks.clone());
    }


    let period = cycle - instant;

    let remaining = (1000000000 - instant) % period;

    for _ in 0..remaining {
        rounded_rocks = move_rock_north(&mut rounded_rocks, &mut cube_shaped_rocks);
        rounded_rocks = move_rock_west(&mut rounded_rocks, &mut cube_shaped_rocks);
        rounded_rocks = move_rock_south(&mut rounded_rocks, &mut cube_shaped_rocks, number_of_columns);
        rounded_rocks = move_rock_east(&mut rounded_rocks, &mut cube_shaped_rocks, number_of_rows);
    }

    total_load(&rounded_rocks, number_of_columns)
}

fn parse_rocks(lines: Vec<String>) -> (HashSet<Position>, HashSet<Position>, Vec<String>) {
    let mut rounded_rocks: HashSet<Position> = HashSet::new();
    let mut cube_shaped_rocks: HashSet<Position> = HashSet::new();
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
    (rounded_rocks, cube_shaped_rocks, lines)
}

fn move_rock_north(rounded_rocks: &HashSet<Position>, cube_shaped_rocks: &HashSet<Position>) -> HashSet<Position> {
    move_rock(rounded_rocks,
              cube_shaped_rocks,
              |Position(_, y)| { *y == 0 },
              |p: &Position| { Position(p.0, p.1 - 1) })
}

fn move_rock_south(rounded_rocks: &HashSet<Position>, cube_shaped_rocks: &HashSet<Position>, number_of_row: usize) -> HashSet<Position> {
    move_rock(rounded_rocks,
              cube_shaped_rocks,
              |Position(_, y)| { *y == number_of_row.clone() - 1 },
              |p: &Position| { Position(p.0, p.1 + 1) })
}

fn move_rock_east(rounded_rocks: &HashSet<Position>, cube_shaped_rocks: &HashSet<Position>, number_of_columns: usize) -> HashSet<Position> {
    move_rock(rounded_rocks,
              cube_shaped_rocks,
              |Position(x, _)| { *x == number_of_columns - 1 },
              |p: &Position| { Position(p.0 + 1, p.1) })
}

fn move_rock_west(rounded_rocks: &HashSet<Position>, cube_shaped_rocks: &HashSet<Position>) -> HashSet<Position> {
    move_rock(rounded_rocks,
              cube_shaped_rocks,
              |Position(x, _)| { *x == 0 },
              |p: &Position| { Position(p.0 - 1, p.1) })
}

fn move_rock(rounded_rocks: &HashSet<Position>,
             cube_shaped_rocks: &HashSet<Position>,
             is_invalid_rock: impl Fn(&Position)-> bool,
             increment: fn(&Position) -> Position) -> HashSet<Position> {
    let mut rounded_rocks = rounded_rocks.clone();
    loop {
        let mut next_rounded_rocks: HashSet<Position> = HashSet::new();
        for Position(x, y) in &rounded_rocks {
            if is_invalid_rock(&Position(*x, *y)) {
                next_rounded_rocks.insert(Position(*x, *y));
                continue;
            }

            let next_position = increment(&Position(*x, *y));
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

    rounded_rocks
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::day14::*;
    use crate::input_reader::read_input_file;

    #[test]
    fn it_solves_first_part() {
        let input = &read_input_file("input_day14.txt");

        assert_eq!(108826, calculate_total_load(input));
    }

    #[test]
    #[ignore] // really slow 1 minute and 7 seconds
    fn it_solves_second_part() {
        let input = &read_input_file("input_day14.txt");

        assert_eq!(99291, calculate_total_load_with_cycle(input));
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

    #[test]
    fn it_calculates_total_load_with_cycle() {
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

        assert_eq!(64, calculate_total_load_with_cycle(input));
    }
}