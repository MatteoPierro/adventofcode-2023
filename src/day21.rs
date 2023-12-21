#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use indoc::indoc;

    use crate::input_reader::{read_input_file, read_lines};

    #[test]
    fn it_solves_first_part() {
        let input = &read_input_file("input_day21.txt");

        assert_eq!(3729, number_of_garden_points(input, 64))
    }

    #[test]
    fn it_calculates_the_number_of_garden_plots() {
        let input = indoc! {"
        ...........
        .....###.#.
        .###.##..#.
        ..#.#...#..
        ....#.#....
        .##..S####.
        .##..#...#.
        .......##..
        .##.#.####.
        .##..##.##.
        ..........."};

        assert_eq!(16, number_of_garden_points(input, 6))
    }

    fn number_of_garden_points(input: &str, iterations: usize) -> usize {
        let rows = read_lines(input);
        let length = rows.len() as isize;
        let width = rows[0].len() as isize;
        let mut rocks: HashSet<(isize, isize)> = HashSet::new();
        let mut garden_plots: HashSet<(isize, isize)> = HashSet::new();
        for (y, row) in rows.iter().enumerate() {
            for (x, c) in row.chars().enumerate() {
                match c {
                    '#' => { rocks.insert((x as isize, y as isize)); }
                    'S' => { garden_plots.insert((x as isize, y as isize)); }
                    _ => {}
                }
            }
        }

        for _ in 0..iterations {
            let mut new_garden_plots: HashSet<(isize, isize)> = HashSet::new();

            for (x, y) in garden_plots {
                for p in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
                    if p.0 < 0 || p.1 < 0 || p.0 >= width || p.1 >= length {
                        continue;
                    }

                    if rocks.contains(&p) {
                        continue;
                    }

                    new_garden_plots.insert(p);
                }
            }

            garden_plots = new_garden_plots;
        }

        garden_plots.len()
    }
}