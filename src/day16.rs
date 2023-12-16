use std::collections::{HashSet, LinkedList};

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
        let mut visited_beams: HashSet<Bean> = HashSet::new();
        let mut bean_to_visit: LinkedList<Bean> = LinkedList::new();
        bean_to_visit.push_back(Bean { tile: self.tile_at(&0, &0).unwrap(), x: 0, y: 0, direction: Right });

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
    fn it_solves_first_puzzle() {
        let input = &read_input_file("input_day16.txt");

        assert_eq!(8021, Contraption::new(input).count_energized_tiles());
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
}