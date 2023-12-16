use std::collections::{HashSet, LinkedList};

use num::range;

use crate::day16::Direction::*;
use crate::input_reader::read_lines;

#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
struct Bean {
    tile: char,
    x: isize,
    y: isize,
    direction: Direction,
}

impl Bean {
    fn step(&self) -> Vec<(isize, isize, Direction)> {
        match self.direction {
            Up => self.step_up(),
            Down => self.step_down(),
            Left => self.step_left(),
            Right => self.step_right()
        }
    }

    fn step_right(&self) -> Vec<(isize, isize, Direction)> {
        match self.tile {
            '.' => vec![(self.x + 1, self.y, Right)],
            '/' => vec![(self.x, self.y - 1, Up)],
            '\\' => vec![(self.x, self.y + 1, Down)],
            '-' => vec![(self.x + 1, self.y, Right)],
            '|' => vec![(self.x, self.y - 1, Up), (self.x, self.y + 1, Down)],
            _ => panic!("unexpected tile!")
        }
    }

    fn step_left(&self) -> Vec<(isize, isize, Direction)> {
        match self.tile {
            '.' => vec![(self.x - 1, self.y, Left)],
            '\\' => vec![(self.x, self.y - 1, Up)],
            '/' => vec![(self.x, self.y + 1, Down)],
            '-' => vec![(self.x - 1, self.y, Left)],
            '|' => vec![(self.x, self.y - 1, Up), (self.x, self.y + 1, Down)],
            _ => panic!("unexpected tile!")
        }
    }

    fn step_up(&self) -> Vec<(isize, isize, Direction)> {
        match self.tile {
            '.' => vec![(self.x, self.y - 1, Up)],
            '/' => vec![(self.x + 1, self.y, Right)],
            '\\' => vec![(self.x - 1, self.y, Left)],
            '-' => vec![(self.x + 1, self.y, Right), (self.x - 1, self.y, Left)],
            '|' => vec![(self.x, self.y - 1, Up)],
            _ => panic!("unexpected tile!")
        }
    }

    fn step_down(&self) -> Vec<(isize, isize, Direction)> {
        match self.tile {
            '.' => vec![(self.x, self.y + 1, Down)],
            '\\' => vec![(self.x + 1, self.y, Right)],
            '/' => vec![(self.x - 1, self.y, Left)],
            '-' => vec![(self.x + 1, self.y, Right), (self.x - 1, self.y, Left)],
            '|' => vec![(self.x, self.y + 1, Down)],
            _ => panic!("unexpected tile!")
        }
    }
}

struct Contraption {
    width: isize,
    length: isize,
    tiles: Vec<Vec<char>>,
}

impl Contraption {
    fn new(input: &str) -> Self {
        let tiles = read_lines(input).iter()
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let length = tiles.len() as isize;
        let width = tiles[0].len() as isize;

        Self { length, width, tiles }
    }

    fn tile_at(&self, x: &isize, y: &isize) -> Option<char> {
        if *x < 0 || *x >= self.width {
            return None;
        }

        if *y < 0 || *y >= self.length {
            return None;
        }

        Some(self.tiles[*y as usize][*x as usize])
    }

    fn evolve(&self, beam: &Bean) -> HashSet<Bean> {
        beam.step().iter().filter_map(|(x, y, direction)| {
            if let Some(tile) = self.tile_at(x, y) {
                Some(Bean { tile, x: *x, y: *y, direction: *direction })
            } else {
                None
            }
        }).collect()
    }

    fn count_energized_tiles(&self) -> usize {
        self.count_energized_tiles_from(Bean { tile: self.tile_at(&0, &0).unwrap(), x: 0, y: 0, direction: Right })
    }

    fn maximize_energized_tiles(&self) -> usize {
        let mut energies: Vec<usize> = vec![];

        for y in range(0, self.length) {
            energies.push(
                self.count_energized_tiles_from(Bean { x: 0, y, direction: Right, tile: self.tile_at(&0, &y).unwrap() })
            );

            energies.push(
                self.count_energized_tiles_from(Bean { x: self.width - 1, y, direction: Left, tile: self.tile_at(&(self.width - 1), &y).unwrap() })
            );
        }

        for x in range(0, self.width) {
            energies.push(
                self.count_energized_tiles_from(Bean { x, y: 0, direction: Down, tile: self.tile_at(&x, &0).unwrap() })
            );

            energies.push(
                self.count_energized_tiles_from(Bean { x, y: self.width - 1, direction: Up, tile: self.tile_at(&x, &(self.width - 1)).unwrap() })
            );
        }

        energies.iter().max().unwrap().clone()
    }

    fn count_energized_tiles_from(&self, bean: Bean) -> usize {
        let mut visited_beams: HashSet<Bean> = HashSet::new();
        let mut bean_to_visit: LinkedList<Bean> = LinkedList::new();
        bean_to_visit.push_back(bean);

        while let Some(bean) = bean_to_visit.pop_front() {
            if visited_beams.contains(&bean) {
                continue;
            }

            visited_beams.insert(bean);
            bean_to_visit.extend(self.evolve(&bean));
        }

        visited_beams.iter()
            .map(|&b| (b.x, b.y))
            .collect::<HashSet<_>>()
            .iter().count()
    }
}


#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::day16::*;
    use crate::input_reader::read_input_file;

    #[test]
    fn it_solves_first_part() {
        let input = &read_input_file("input_day16.txt");

        assert_eq!(8021, Contraption::new(input).count_energized_tiles());
    }

    #[test]
    fn it_solves_second_part() {
        let input = &read_input_file("input_day16.txt");

        assert_eq!(8216, Contraption::new(input).maximize_energized_tiles());
    }

    #[test]
    fn it_counts_energized_tiles() {
        let input = indoc! {"
        .|...\\....
        |.-.\\.....
        .....|-...
        ........|.
        ..........
        .........\\
        ..../.\\\\..
        .-.-/..|..
        .|....-|.\\
        ..//.|...."};

        assert_eq!(46, Contraption::new(input).count_energized_tiles());
    }

    #[test]
    fn it_maximizes_energized_tiles() {
        let input = indoc! {"
        .|...\\....
        |.-.\\.....
        .....|-...
        ........|.
        ..........
        .........\\
        ..../.\\\\..
        .-.-/..|..
        .|....-|.\\
        ..//.|...."};

        assert_eq!(51, Contraption::new(input).maximize_energized_tiles());
    }
}