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
                    continue;
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
    fn it_solves_second_part() {
        let input = &read_input_file("input_day21.txt");

        let grid = Grid::build_from(input);
        // grid is a square
        assert_eq!(grid.length, grid.width);
        let size = grid.length;

        // star is in the middle of the grid
        let start = grid.start;
        assert_eq!(start.0, start.1);
        assert_eq!(start.0, size / 2);

        // best case we can arrive at the middle of a grid
        let steps: isize = 26501365;
        assert_eq!(steps % size, size / 2);

        let grid_width = (steps / size - 1) as usize;

        let odd = (grid_width / 2 * 2 + 1).pow(2);
        let even = ((grid_width + 1) / 2 * 2).pow(2);

        let odd_points = grid.fill(start, (size + 2) as usize);
        let even_points = grid.fill(start, (size + 1) as usize);

        let corner_t = grid.fill((start.0, size - 1), (size - 1) as usize);
        let corner_r = grid.fill((size - 1, start.1), (size - 1) as usize);
        let corner_l = grid.fill((start.0, 0), (size - 1) as usize);
        let corner_b = grid.fill((0, start.1), (size - 1) as usize);

        let small_tr = grid.fill((size - 1, 0), (size / 2 - 1) as usize);
        let small_tl = grid.fill((size - 1, size - 1), (size / 2 - 1) as usize);
        let small_br = grid.fill((0, 0), (size / 2 - 1) as usize);
        let small_bl = grid.fill((0, size - 1), (size / 2 - 1) as usize);

        let large_tr = grid.fill((size - 1, 0), (size * 3 / 2 - 1) as usize);
        let large_tl = grid.fill((size - 1, size - 1), (size * 3 / 2 - 1) as usize);
        let large_br = grid.fill((0, 0), (size * 3 / 2 - 1) as usize);
        let large_bl = grid.fill((0, size - 1), (size * 3 / 2 - 1) as usize);

        let total = odd * odd_points +
            even * even_points +
            corner_t + corner_r + corner_b + corner_l +
            (grid_width + 1) * (small_tr + small_tl + small_br + small_bl) +
            grid_width * (large_tr + large_tl + large_br + large_bl);

        assert_eq!(621289922886149, total);
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