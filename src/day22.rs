use std::cmp::{max, min};

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Position {
    x: usize,
    y: usize,
    z: usize,
}

impl Position {
    fn new(raw_position: &str) -> Self {
        let coords = raw_position.split(",").map(|c| c.parse::<usize>().unwrap()).collect::<Vec<_>>();
        Position { x: coords[0], y: coords[1], z: coords[2] }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Brick {
    start: Position,
    end: Position,
}

fn free_fall(bricks: &mut Vec<Brick>) {
    bricks.sort_by(|b1, b2| b1.start.z.cmp(&b2.start.z));

    for current_index in 0..bricks.len() {
        let current = &bricks[current_index];
        let mut z_max = 0;
        for prev_index in 0..current_index {
            let prev = &bricks[prev_index];

            if current.is_overlapped_by(prev) {
                z_max = max(z_max, prev.end.z + 1);
            }
        }

        let current = &mut bricks[current_index];
        current.update_z(z_max);
    }

    bricks.sort_by(|b1, b2| b1.start.z.cmp(&b2.start.z));
}

impl Brick {
    fn is_overlapped_by(&self, other: &Self) -> bool {
        max(self.start.x, other.start.x) <= min(self.end.x, other.end.x) &&
            max(self.start.y, other.start.y) <= min(self.end.y, other.end.y)
    }

    fn update_z(&mut self, new_z: usize) {
        let diff = self.end.z - self.start.z;
        self.start.z = new_z;
        self.end.z = new_z + diff;
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};
    use indoc::indoc;
    use crate::day22::*;
    use crate::input_reader::{read_input_file, read_lines};

    #[test]
    fn it_solves_first_puzzle() {
        let input = &read_input_file("input_day22.txt");

        assert_eq!(527, count_disintegrable_bricks(input));
    }

    #[test]
    fn it_counts_disintegrable_bricks() {
        let input = indoc! {"
            1,0,1~1,2,1
            0,0,2~2,0,2
            0,2,3~2,2,3
            0,0,4~0,2,4
            2,0,5~2,2,5
            0,1,6~2,1,6
            1,1,8~1,1,9"};

        assert_eq!(5, count_disintegrable_bricks(input));
    }

    fn count_disintegrable_bricks(input: &str) -> usize {
        let mut bricks = read_lines(input).iter().map(|line| {
            let parts = line.split("~").collect::<Vec<_>>();
            Brick { start: Position::new(parts[0]), end: Position::new(parts[1]) }
        }).collect::<Vec<_>>();

        free_fall(&mut bricks);

        let mut supporting_bricks: HashMap<Brick, HashSet<Brick>> = HashMap::new();
        let mut supported_bricks: HashMap<Brick, HashSet<Brick>> = HashMap::new();
        for current_index in 0..bricks.len() {
            let current = &bricks[current_index];
            let mut current_supported_bricks = HashSet::new();
            let mut next_index = current_index + 1;
            while next_index < bricks.len() {
                let next = &bricks[next_index];
                if next.start.z > current.end.z + 1 {
                    break;
                }

                if next.is_overlapped_by(current) {
                    current_supported_bricks.insert((*next).clone());
                }

                next_index += 1;
            }

            for b in &current_supported_bricks {
                let current_supporting_bricks = supported_bricks.entry((*b).clone()).or_insert(HashSet::new());
                current_supporting_bricks.insert((*current).clone());
            }

            supporting_bricks.insert((*current).clone(), current_supported_bricks);
        }

        let mut count: usize = 0;
        for b in supporting_bricks.keys() {
            if supporting_bricks[b].iter().all(|sb| supported_bricks[sb].len() >= 2) {
                count += 1;
            }
        }

        count
    }
}