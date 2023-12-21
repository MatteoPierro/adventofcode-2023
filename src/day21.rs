use std::collections::{HashSet, LinkedList};

use num::Integer;

use crate::input_reader::read_lines;

struct Grid {
    rocks: HashSet<(isize, isize)>,
    start: (isize, isize),
    length: isize,
    width: isize,
}

impl Grid {
    fn build_from(input: &str) -> Self {
        let rows = read_lines(input);
        let length = rows.len() as isize;
        let width = rows[0].len() as isize;
        let mut rocks: HashSet<(isize, isize)> = HashSet::new();
        let mut starting: Option<(isize, isize)> = None;
        for (y, row) in rows.iter().enumerate() {
            for (x, c) in row.chars().enumerate() {
                match c {
                    '#' => { rocks.insert((x as isize, y as isize)); }
                    'S' => { starting = Some((x as isize, y as isize)) }
                    _ => {}
                }
            }
        }

        Grid { rocks, start: starting.unwrap(), length, width }
    }

    fn fill(&self, start: (isize, isize), steps: usize) -> usize {
        let mut garden_plots: HashSet<(isize, isize)> = HashSet::new();
        let mut seen: HashSet<(isize, isize)> = HashSet::new();
        seen.insert(start);

        let mut queue: LinkedList<((isize, isize), usize)> = LinkedList::new();
        queue.push_back((start, steps));

        while let Some(((x, y), remaining_steps)) = queue.pop_front() {
            if remaining_steps.is_even() {
                garden_plots.insert((x, y));
            }

            if remaining_steps == 0 {
                continue;
            }

            for p in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
                if p.0 < 0 || p.1 < 0 || p.0 >= self.width || p.1 >= self.length {
                    continue;
                }

                if self.rocks.contains(&p) {
                    continue;
                }

                if seen.contains(&p) {
                    continue
                }

                seen.insert(p);
                queue.push_back((p, remaining_steps - 1));
            }
        }

        garden_plots.len()
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::day21::*;
    use crate::input_reader::read_input_file;

    #[test]
    fn it_solves_first_part() {
        let input = &read_input_file("input_day21.txt");

        assert_eq!(3729, Grid::build_from(input).fill(Grid::build_from(input).start, 64))
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

        assert_eq!(16, Grid::build_from(input).fill(Grid::build_from(input).start, 6))
    }
}