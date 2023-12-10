use std::ops::Div;
use itertools::Itertools;

use crate::day10::Direction::*;
use crate::input_reader::read_lines;

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
struct Position(usize, usize);

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord)]
struct Animal {
    position: Position,
    direction: Direction,
}

fn walk(animal: &Animal, maze: &Vec<Vec<char>>) -> Option<Animal> {
    let c = maze[animal.position.1][animal.position.0];
    match c {
        '|' => walk_from_vertical_pipe(animal),
        '-' => walk_from_horizontal_pipe(animal),
        'L' => walk_from_l(animal),
        'J' => walk_from_j(animal),
        '7' => walk_from_7(animal),
        'F' => walk_from_f(animal),
        '.' => walk_from_dot(animal),
        _ => panic!("unknown tile")
    }
}

fn walk_from_start(animal: &Animal) -> Option<Animal> {
    let animal = match animal.direction {
        North => Animal {
            position: Position(animal.position.0, animal.position.1 - 1),
            direction: North,
        },
        South => Animal {
            position: Position(animal.position.0, animal.position.1 + 1),
            direction: South,
        },
        East => Animal {
            position: Position(animal.position.0 + 1, animal.position.1),
            direction: East,
        },
        West => Animal {
            position: Position(animal.position.0 - 1, animal.position.1),
            direction: West,
        }
    };

    Some(animal)
}

fn walk_from_dot(_animal: &Animal) -> Option<Animal> {
    None
}

fn walk_from_l(animal: &Animal) -> Option<Animal> {
    match animal.direction {
        South => Some(Animal {
            position: Position(animal.position.0 + 1, animal.position.1),
            direction: East,
        }),
        West => Some(Animal {
            position: Position(animal.position.0, animal.position.1 - 1),
            direction: North,
        }),
        _ => None
    }
}

fn walk_from_j(animal: &Animal) -> Option<Animal> {
    match animal.direction {
        South => Some(Animal {
            position: Position(animal.position.0 - 1, animal.position.1),
            direction: West,
        }),
        East => Some(Animal {
            position: Position(animal.position.0, animal.position.1 - 1),
            direction: North,
        }),
        _ => None
    }
}

fn walk_from_7(animal: &Animal) -> Option<Animal> {
    match animal.direction {
        North => Some(Animal {
            position: Position(animal.position.0 - 1, animal.position.1),
            direction: West,
        }),
        East => Some(Animal {
            position: Position(animal.position.0, animal.position.1 + 1),
            direction: South,
        }),
        _ => None
    }
}

fn walk_from_f(animal: &Animal) -> Option<Animal> {
    match animal.direction {
        North => Some(Animal {
            position: Position(animal.position.0 + 1, animal.position.1),
            direction: East,
        }),
        West => Some(Animal {
            position: Position(animal.position.0, animal.position.1 + 1),
            direction: South,
        }),
        _ => None
    }
}

fn walk_from_vertical_pipe(animal: &Animal) -> Option<Animal> {
    match animal.direction {
        North => Some(Animal {
            position: Position(animal.position.0, animal.position.1 - 1),
            direction: North,
        }),
        South => Some(Animal {
            position: Position(animal.position.0, animal.position.1 + 1),
            direction: South,
        }),
        _ => None
    }
}

fn walk_from_horizontal_pipe(animal: &Animal) -> Option<Animal> {
    match animal.direction {
        West => Some(Animal {
            position: Position(animal.position.0 - 1, animal.position.1),
            direction: West,
        }),
        East => Some(Animal {
            position: Position(animal.position.0 + 1, animal.position.1),
            direction: East,
        }),
        _ => None
    }
}

fn calculate_steps(input: &str) -> usize {
    find_polygon(input).iter().count().div(2)
}

fn find_polygon(input: &str) -> Vec<Position> {
    let maze: Vec<_> = parse_maze(input);

    let starting_position = find_starting_position(&maze);

    let mut polygon = vec![starting_position];

    let mut current = start_walking(&maze, starting_position);
    polygon.push(current.position);

    while current.position != starting_position {
        current = walk(&current, &maze).unwrap();
        polygon.push(current.position);
    }

    polygon
}

fn start_walking(maze: &Vec<Vec<char>>, starting_position: Position) -> Animal {
    [North, South, East, West]
        .iter()
        .map(|&direction| Animal { position: starting_position, direction })
        .filter_map(|animal| walk_from_start(&animal))
        .find_or_first(|animal| walk(&animal, &maze) != None)
        .unwrap()
}

fn find_starting_position(maze: &Vec<Vec<char>>) -> Position {
    maze.iter().enumerate()
        .find_map(|(y, line)| {
            line.iter().position(|&c| c == 'S').map(|x| Position(x, y))
        }).unwrap()
}

fn parse_maze(input: &str) -> Vec<Vec<char>> {
    read_lines(input).iter()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect()
}

// Calculate enclosed point using Pick's theorem
// https://en.wikipedia.org/wiki/Pick's_theorem
fn count_enclosed_points(input: &str) -> isize {
    let polygon = find_polygon(input);
    let area = calculate_area(&polygon);
    let b = polygon.iter().count() as isize;
    area + 1 - (b / 2)
}

// Calculate area using the Shoelace formula
// https://en.wikipedia.org/wiki/Shoelace_formula
fn calculate_area(polygon: &Vec<Position>) -> isize {
    polygon.windows(2)
        .map(|positions| {
            let Position(x1, y1) = positions[0];
            let Position(x2, y2) = positions[1];
            (x1 * y2) as isize - (x2 * y1) as isize
        })
        .sum::<isize>()
        .abs()
        .div(2)
}


#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::day10::*;
    use crate::input_reader::read_input_file;

    #[test]
    fn it_solves_first_part() {
        let input = read_input_file("input_day10.txt");
        assert_eq!(6812, calculate_steps(&input));
    }

    #[test]
    fn it_solves_second_part() {
        let input = read_input_file("input_day10.txt");

        assert_eq!(527, count_enclosed_points(&input));
    }

    #[test]
    fn it_finds_polygon() {
        let input = indoc! {"
        ...........
        .S-------7.
        .|F-----7|.
        .||.....||.
        .||.....||.
        .|L-7.F-J|.
        .|..|.|..|.
        .L--J.L--J.
        ..........."};

        assert_eq!(4, count_enclosed_points(input));
    }

    #[test]
    fn it_calculate_steps() {
        let input = indoc! {"
        .....
        .S-7.
        .|.|.
        .L-J.
        ....."};
        assert_eq!(4, calculate_steps(input));

        let input = indoc! {"
        ...F7.
        ..FJ|.
        .SJ.L7
        .|F--J
        .LJ..."};
        assert_eq!(8, calculate_steps(input));
    }
}