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
    neighbours_finder: fn(&CityMap, &State) -> Vec<State>,
    end_condition: fn((usize, usize), &State) -> bool,
}

impl CityMap {
    fn build(input: &str) -> Self {
        Self::new(
            input,
            neighbours,
            |target, current| target == current.position,
        )
    }

    fn build_for_crucibles(input: &str) -> Self {
        Self::new(
            input,
            neighbour_crucibles,
            |target, current| target == current.position && current.steps >= 4
        )
    }

    fn new(input: &str,
           finder: fn(&CityMap, &State) -> Vec<State>,
           end_condition: fn((usize, usize), &State) -> bool) -> Self {
        let mut city_blocks: Vec<Vec<usize>> = vec![];
        for row in read_lines(input) {
            let mut parsed_row: Vec<usize> = vec![];
            for c in row.chars() {
                parsed_row.push(c.to_digit(10).unwrap() as usize);
            }
            city_blocks.push(parsed_row);
        }

        CityMap {
            city_blocks: city_blocks.clone(),
            length: city_blocks.len(),
            width: city_blocks[0].len(),
            neighbours_finder: finder,
            end_condition,
        }
    }

    fn initial_heat_losses(&self) -> HashMap<(usize, usize, Direction, usize), usize> {
        let mut distances = HashMap::new();

        for y in 0..self.city_blocks.len() {
            for x in 0..self.city_blocks[y].len() {
                for steps in 1..=10 {
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

    fn minimize_heat_loss(&self) -> usize {
        let target = self.target();
        let mut heat_losses = self.initial_heat_losses();
        let mut pred = self.initial_pred();

        let mut heap: BinaryHeap<State> = BinaryHeap::new();

        heap.push(State { heat_lost: self.heat_at(1, 0), position: (1, 0), steps: 1, direction: Right });
        heap.push(State { heat_lost: self.heat_at(0, 1), position: (0, 1), steps: 1, direction: Down });

        while let Some(state) = heap.pop() {
            if (self.end_condition)(target, &state) {
                return state.heat_lost;
            }

            if state.heat_lost > heat_losses[&(state.position.0, state.position.1, state.direction, state.steps)] {
                continue;
            }

            for next_state in (self.neighbours_finder)(self, &state) {
                if next_state.heat_lost >= heat_losses[&(next_state.position.0, next_state.position.1, next_state.direction, next_state.steps)] {
                    continue;
                }

                *heat_losses.get_mut(&(next_state.position.0, next_state.position.1, next_state.direction, next_state.steps)).unwrap() = next_state.heat_lost;
                *pred.get_mut(&next_state.position).unwrap() = Some(state.position);
                heap.push(next_state);
            }
        }

        panic!("NOT FOUND!")
    }
}

fn neighbours(map: &CityMap, current_state: &State) -> Vec<State> {
    match current_state.direction {
        Up => neighbours_up(map, current_state),
        Down => neighbours_down(map, current_state),
        Left => neighbours_left(map, current_state),
        Right => neighbours_right(map, current_state)
    }
}

fn neighbours_up(map: &CityMap, current_state: &State) -> Vec<State> {
    let mut next_state = vec![];

    if current_state.position.1 > 0 && current_state.steps < 3 {
        next_state.push(State {
            direction: Up,
            position: (current_state.position.0, current_state.position.1 - 1),
            steps: current_state.steps + 1,
            heat_lost: current_state.heat_lost + map.heat_at(current_state.position.0, current_state.position.1 - 1),
        })
    }

    if current_state.position.0 > 0 {
        next_state.push(State {
            direction: Left,
            position: (current_state.position.0 - 1, current_state.position.1),
            steps: 1,
            heat_lost: current_state.heat_lost + map.heat_at(current_state.position.0 - 1, current_state.position.1),
        })
    }

    if current_state.position.0 < map.width - 1 {
        next_state.push(State {
            direction: Right,
            position: (current_state.position.0 + 1, current_state.position.1),
            steps: 1,
            heat_lost: current_state.heat_lost + map.heat_at(current_state.position.0 + 1, current_state.position.1),
        })
    }

    next_state
}

fn neighbours_down(map: &CityMap, current_state: &State) -> Vec<State> {
    let mut next_state = vec![];

    if current_state.position.1 < map.length - 1 && current_state.steps < 3 {
        next_state.push(State {
            direction: Down,
            position: (current_state.position.0, current_state.position.1 + 1),
            steps: current_state.steps + 1,
            heat_lost: current_state.heat_lost + map.heat_at(current_state.position.0, current_state.position.1 + 1),
        })
    }

    if current_state.position.0 > 0 {
        next_state.push(State {
            direction: Left,
            position: (current_state.position.0 - 1, current_state.position.1),
            steps: 1,
            heat_lost: current_state.heat_lost + map.heat_at(current_state.position.0 - 1, current_state.position.1),
        })
    }

    if current_state.position.0 < map.width - 1 {
        next_state.push(State {
            direction: Right,
            position: (current_state.position.0 + 1, current_state.position.1),
            steps: 1,
            heat_lost: current_state.heat_lost + map.heat_at(current_state.position.0 + 1, current_state.position.1),
        })
    }

    next_state
}

fn neighbours_right(map: &CityMap, current_state: &State) -> Vec<State> {
    let mut next_state = vec![];

    if current_state.position.0 < map.width - 1 && current_state.steps < 3 {
        next_state.push(State {
            direction: Right,
            position: (current_state.position.0 + 1, current_state.position.1),
            steps: current_state.steps + 1,
            heat_lost: current_state.heat_lost + map.heat_at(current_state.position.0 + 1, current_state.position.1),
        })
    }

    if current_state.position.1 < map.length - 1 {
        next_state.push(State {
            direction: Down,
            position: (current_state.position.0, current_state.position.1 + 1),
            steps: 1,
            heat_lost: current_state.heat_lost + map.heat_at(current_state.position.0, current_state.position.1 + 1),
        })
    }

    if current_state.position.1 > 0 {
        next_state.push(State {
            direction: Up,
            position: (current_state.position.0, current_state.position.1 - 1),
            steps: 1,
            heat_lost: current_state.heat_lost + map.heat_at(current_state.position.0, current_state.position.1 - 1),
        })
    }

    next_state
}

fn neighbours_left(map: &CityMap, current_state: &State) -> Vec<State> {
    let mut next_state = vec![];

    if current_state.position.0 > 0 && current_state.steps < 3 {
        next_state.push(State {
            direction: Left,
            position: (current_state.position.0 - 1, current_state.position.1),
            steps: current_state.steps + 1,
            heat_lost: current_state.heat_lost + map.heat_at(current_state.position.0 - 1, current_state.position.1),
        })
    }

    if current_state.position.1 < map.length - 1 {
        next_state.push(State {
            direction: Down,
            position: (current_state.position.0, current_state.position.1 + 1),
            steps: 1,
            heat_lost: current_state.heat_lost + map.heat_at(current_state.position.0, current_state.position.1 + 1),
        })
    }

    if current_state.position.1 > 0 {
        next_state.push(State {
            direction: Up,
            position: (current_state.position.0, current_state.position.1 - 1),
            steps: 1,
            heat_lost: current_state.heat_lost + map.heat_at(current_state.position.0, current_state.position.1 - 1),
        })
    }

    next_state
}

fn neighbour_crucibles(map: &CityMap, current_state: &State) -> Vec<State> {
    let directions: Vec<Direction> = if current_state.steps < 4 {
        vec![current_state.direction]
    } else {
        vec![Right, Left, Up, Down]
    };

    let mut possible_neighbours = vec![];

    for direction in directions {
        if current_state.direction == Up && direction == Down {
            continue;
        }

        if current_state.direction == Down && direction == Up {
            continue;
        }

        if current_state.direction == Left && direction == Right {
            continue;
        }

        if current_state.direction == Right && direction == Left {
            continue;
        }

        if current_state.steps == 10 && direction == current_state.direction {
            continue;
        }

        if let Some(new_position) = find_new_position(map, direction, current_state.position) {
            possible_neighbours.push(
                State {
                    steps: if direction == current_state.direction { current_state.steps + 1 } else { 1 },
                    heat_lost: current_state.heat_lost + map.heat_at(new_position.0, new_position.1),
                    position: new_position,
                    direction,
                }
            )
        }
    }

    possible_neighbours
}

fn find_new_position(map: &CityMap, direction: Direction, position: (usize, usize)) -> Option<(usize, usize)> {
    if direction == Up && position.1 > 0 {
        return Some((position.0, position.1 - 1));
    }

    if direction == Down && position.1 < map.length - 1 {
        return Some((position.0, position.1 + 1));
    }

    if direction == Left && position.0 > 0 {
        return Some((position.0 - 1, position.1));
    }

    if direction == Right && position.0 < map.width - 1 {
        return Some((position.0 + 1, position.1));
    }

    None
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::day17::*;
    use crate::input_reader::read_input_file;

    #[test]
    #[ignore] // runs in ~ 1,5 seconds
    fn it_solves_first_part() {
        let input = &read_input_file("input_day17.txt");

        assert_eq!(1138, CityMap::build(input).minimize_heat_loss())
    }

    #[test]
    #[ignore] // runs in ~ 2,5 seconds
    fn it_solves_second_part() {
        let input = &read_input_file("input_day17.txt");

        assert_eq!(1312, CityMap::build_for_crucibles(input).minimize_heat_loss())
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

        assert_eq!(102, CityMap::build(input).minimize_heat_loss())
    }

    #[test]
    fn it_minimizes_heat_loss_in_crucibles() {
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

        assert_eq!(94, CityMap::build_for_crucibles(input).minimize_heat_loss());

        let input = indoc! {"
        111111111111
        999999999991
        999999999991
        999999999991
        999999999991"};
        assert_eq!(71, CityMap::build_for_crucibles(input).minimize_heat_loss());
    }
}