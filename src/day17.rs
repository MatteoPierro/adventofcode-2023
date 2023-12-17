use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

use crate::day17::Direction::*;
use crate::input_reader::read_lines;

#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    heat_lost: usize,
    position: (usize, usize),
    steps: usize,
    direction: Direction,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.heat_lost.cmp(&self.heat_lost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct CityMap {
    city_blocks: Vec<Vec<usize>>,
    length: usize,
    width: usize,
}

impl CityMap {
    fn new(input: &str) -> Self {
        let mut city_blocks: Vec<Vec<usize>> = vec![];
        for row in read_lines(input) {
            let mut parsed_row: Vec<usize> = vec![];
            for c in row.chars() {
                parsed_row.push(c.to_digit(10).unwrap() as usize);
            }
            city_blocks.push(parsed_row);
        }

        CityMap { city_blocks: city_blocks.clone(), length: city_blocks.len(), width: city_blocks[0].len() }
    }
    fn initial_heat_losses(&self) -> HashMap<(usize, usize, Direction, usize), usize> {
        let mut distances = HashMap::new();

        for y in 0..self.city_blocks.len() {
            for x in 0..self.city_blocks[y].len() {
                for steps in 1..=3 {
                    distances.insert((x, y, Up, steps), usize::MAX);
                    distances.insert((x, y, Down, steps), usize::MAX);
                    distances.insert((x, y, Left, steps), usize::MAX);
                    distances.insert((x, y, Right, steps), usize::MAX);
                }
            }
        }

        distances
    }

    fn initial_pred(&self) -> HashMap<(usize, usize), Option<(usize, usize)>> {
        let mut pred = HashMap::new();

        for y in 0..self.city_blocks.len() {
            for x in 0..self.city_blocks[y].len() {
                pred.insert((x, y), None);
            }
        }

        pred
    }

    fn target(&self) -> (usize, usize) {
        (self.width - 1, self.length - 1)
    }

    fn heat_at(&self, x: usize, y: usize) -> usize {
        self.city_blocks[y][x]
    }

    fn neighbours(&self, current_state: &State) -> Vec<State> {
        match current_state.direction {
            Up => self.neighbours_up(current_state),
            Down => self.neighbours_down(current_state),
            Left => self.neighbours_left(current_state),
            Right => self.neighbours_right(current_state)
        }
    }

    fn neighbours_up(&self, current_state: &State) -> Vec<State> {
        let mut next_state = vec![];

        if current_state.position.1 > 0 && current_state.steps < 3 {
            next_state.push(State {
                direction: Up,
                position: (current_state.position.0, current_state.position.1 - 1),
                steps: current_state.steps + 1,
                heat_lost: current_state.heat_lost + self.heat_at(current_state.position.0, current_state.position.1 - 1),
            })
        }

        if current_state.position.0 > 0 {
            next_state.push(State {
                direction: Left,
                position: (current_state.position.0 - 1, current_state.position.1),
                steps: 1,
                heat_lost: current_state.heat_lost + self.heat_at(current_state.position.0 - 1, current_state.position.1),
            })
        }

        if current_state.position.0 < self.width - 1 {
            next_state.push(State {
                direction: Right,
                position: (current_state.position.0 + 1, current_state.position.1),
                steps: 1,
                heat_lost: current_state.heat_lost + self.heat_at(current_state.position.0 + 1, current_state.position.1),
            })
        }

        next_state
    }

    fn neighbours_down(&self, current_state: &State) -> Vec<State> {
        let mut next_state = vec![];

        if current_state.position.1 < self.length - 1 && current_state.steps < 3 {
            next_state.push(State {
                direction: Down,
                position: (current_state.position.0, current_state.position.1 + 1),
                steps: current_state.steps + 1,
                heat_lost: current_state.heat_lost + self.heat_at(current_state.position.0, current_state.position.1 + 1),
            })
        }

        if current_state.position.0 > 0 {
            next_state.push(State {
                direction: Left,
                position: (current_state.position.0 - 1, current_state.position.1),
                steps: 1,
                heat_lost: current_state.heat_lost + self.heat_at(current_state.position.0 - 1, current_state.position.1),
            })
        }

        if current_state.position.0 < self.width - 1 {
            next_state.push(State {
                direction: Right,
                position: (current_state.position.0 + 1, current_state.position.1),
                steps: 1,
                heat_lost: current_state.heat_lost + self.heat_at(current_state.position.0 + 1, current_state.position.1),
            })
        }

        next_state
    }

    fn neighbours_right(&self, current_state: &State) -> Vec<State> {
        let mut next_state = vec![];

        if current_state.position.0 < self.width - 1 && current_state.steps < 3 {
            next_state.push(State {
                direction: Right,
                position: (current_state.position.0 + 1, current_state.position.1),
                steps: current_state.steps + 1,
                heat_lost: current_state.heat_lost + self.heat_at(current_state.position.0 + 1, current_state.position.1),
            })
        }

        if current_state.position.1 < self.length - 1 {
            next_state.push(State {
                direction: Down,
                position: (current_state.position.0, current_state.position.1 + 1),
                steps: 1,
                heat_lost: current_state.heat_lost + self.heat_at(current_state.position.0, current_state.position.1 + 1),
            })
        }

        if current_state.position.1 > 0 {
            next_state.push(State {
                direction: Up,
                position: (current_state.position.0, current_state.position.1 - 1),
                steps: 1,
                heat_lost: current_state.heat_lost + self.heat_at(current_state.position.0, current_state.position.1 - 1),
            })
        }

        next_state
    }

    fn neighbours_left(&self, current_state: &State) -> Vec<State> {
        let mut next_state = vec![];

        if current_state.position.0 > 0 && current_state.steps < 3 {
            next_state.push(State {
                direction: Left,
                position: (current_state.position.0 - 1, current_state.position.1),
                steps: current_state.steps + 1,
                heat_lost: current_state.heat_lost + self.heat_at(current_state.position.0 - 1, current_state.position.1),
            })
        }

        if current_state.position.1 < self.length - 1 {
            next_state.push(State {
                direction: Down,
                position: (current_state.position.0, current_state.position.1 + 1),
                steps: 1,
                heat_lost: current_state.heat_lost + self.heat_at(current_state.position.0, current_state.position.1 + 1),
            })
        }

        if current_state.position.1 > 0 {
            next_state.push(State {
                direction: Up,
                position: (current_state.position.0, current_state.position.1 - 1),
                steps: 1,
                heat_lost: current_state.heat_lost + self.heat_at(current_state.position.0, current_state.position.1 - 1),
            })
        }

        next_state
    }

    fn minimize_heat_loss(&self) -> usize {
        let target = self.target();
        let mut heat_losses = self.initial_heat_losses();
        let mut pred = self.initial_pred();

        let mut heap: BinaryHeap<State> = BinaryHeap::new();

        // *heat_losses.get_mut(&(0, 0, Right, 0)).unwrap() = 0;
        // *heat_losses.get_mut(&(0, 0, Right, 0)).unwrap() = 0;
        heap.push(State { heat_lost: self.heat_at(1, 0), position: (1, 0), steps: 1, direction: Right });
        heap.push(State { heat_lost: self.heat_at(0, 1), position: (0, 1), steps: 1, direction: Down });

        while let Some(state) = heap.pop() {
            if state.position == target {
                return state.heat_lost;
            }

            if state.heat_lost > heat_losses[&(state.position.0, state.position.1, state.direction, state.steps)] {
                continue;
            }

            for next_state in self.neighbours(&state) {
                if next_state.heat_lost >= heat_losses[&(next_state.position.0, next_state.position.1, next_state.direction, next_state.steps)] {
                    continue;
                }

                *heat_losses.get_mut(&(next_state.position.0, next_state.position.1, next_state.direction, next_state.steps)).unwrap() = next_state.heat_lost;
                *pred.get_mut(&next_state.position).unwrap() = Some(state.position);
                heap.push(next_state);
            }
        }

        // println!("{:?}", heat_losses.get(&target));
        //
        // println!("{:?}", pred.get(&target).unwrap().unwrap());
        // let mut p = target;
        //
        // while let Some(x) = pred.get(&p).unwrap() {
        //     println!("{:?}", x);
        //     p = *x;
        // }

        // *heat_losses.get(&target).unwrap()
        panic!("NOT FOUND!")
    }
}


#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::day17::*;
    use crate::input_reader::read_input_file;

    #[test]
    fn it_solves_first_part() {
        let input = &read_input_file("input_day17.txt");

        assert_eq!(1138, CityMap::new(input).minimize_heat_loss())
    }

    #[test]
    fn it_minimizes_heat_loss() {
        let input = indoc! {"
        2413432311323
        3215453535623
        3255245654254
        3446585845452
        4546657867536
        1438598798454
        4457876987766
        3637877979653
        4654967986887
        4564679986453
        1224686865563
        2546548887735
        4322674655533"};

        assert_eq!(102, CityMap::new(input).minimize_heat_loss())
    }
}