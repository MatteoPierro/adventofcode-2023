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

trait Tile {
    fn walk(animal: &Animal) -> Option<Animal>;
}

struct Start;

impl Tile for Start {
    fn walk(animal: &Animal) -> Option<Animal> {
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
}

struct Dot;

impl Tile for Dot {
    fn walk(_animal: &Animal) -> Option<Animal> {
        None
    }
}

struct L;

impl Tile for L {
    fn walk(animal: &Animal) -> Option<Animal> {
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
}

struct J;

impl Tile for J {
    fn walk(animal: &Animal) -> Option<Animal> {
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
}

struct Seven;

impl Tile for Seven {
    fn walk(animal: &Animal) -> Option<Animal> {
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
}

struct F;

impl Tile for F {
    fn walk(animal: &Animal) -> Option<Animal> {
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
}

struct VerticalPipe;

impl Tile for VerticalPipe {
    fn walk(animal: &Animal) -> Option<Animal> {
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
}

struct HorizontalPipe;

impl Tile for HorizontalPipe {
    fn walk(animal: &Animal) -> Option<Animal> {
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
}

fn walk(animal: &Animal, maze: &Vec<Vec<char>>) -> Option<Animal> {
    let c = maze[animal.position.1][animal.position.0];
    match c {
        '|' => VerticalPipe::walk(animal),
        '-' => HorizontalPipe::walk(animal),
        'L' => L::walk(animal),
        'J' => J::walk(animal),
        '7' => Seven::walk(animal),
        'F' => F::walk(animal),
        '.' => Dot::walk(animal),
        _ => panic!("unknown tile")
    }
}

fn calculate_steps(input: &str) -> usize {
    let maze: Vec<_> = read_lines(input).iter()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();

    let position = maze.iter().enumerate()
        .find_map(|(y, line)| {
            line.iter().position(|&c| c == 'S').map(|x| Position(x, y))
        }).unwrap();

    let after_first_step: Vec<_> = [North, South, East, West]
        .iter()
        .map(|&direction| Animal { position, direction })
        .filter_map(|animal| Start::walk(&animal))
        .flat_map(|animal| walk(&animal, &maze))
        .collect();

    if after_first_step.iter().count() != 2 {
        panic!("there should be only two starting point")
    }

    let mut first: Animal = after_first_step.first().unwrap().clone();
    let mut second: Animal = after_first_step.last().unwrap().clone();
    let mut steps: usize = 2;

    while first.position != second.position {
        first = walk(&first, &maze).unwrap();
        second = walk(&second, &maze).unwrap();
        steps += 1;
    }
    steps
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