use itertools::Itertools;

use crate::input_reader::read_lines;

#[derive(Debug, Clone, PartialEq)]
struct Galaxy {
    index: usize,
    x: usize,
    y: usize,
}

impl Galaxy {
    fn expand_row(&mut self, expansion: usize) {
        self.y += expansion;
    }

    fn expand_column(&mut self, expansion: usize) {
        self.x += expansion;
    }

    fn distance(&self, other: &Galaxy) -> usize {
        ((self.y as isize - other.y as isize).abs() + (self.x as isize - other.x as isize).abs()) as usize
    }
}

fn calculate_sum_distances(galaxies: Vec<Galaxy>) -> usize {
    galaxies.into_iter().combinations(2)
        .map(|combination| {
            let first = combination.first().unwrap();
            let second = combination.last().unwrap();
            first.distance(second)
        }).sum()
}

fn expand_galaxies(input: &str, expansion_factor: usize) -> Vec<Galaxy> {
    let mut galaxies = parse_galaxies(input);

    expand_galaxies_along_one_dimension(
        &mut galaxies,
        expansion_factor,
        |galaxy: &Galaxy| galaxy.y,
        |g: &mut Galaxy, expansion: usize| g.expand_row(expansion),
    );

    expand_galaxies_along_one_dimension(
        &mut galaxies,
        expansion_factor,
        |galaxy: &Galaxy| galaxy.x,
        |g: &mut Galaxy, expansion: usize| g.expand_column(expansion),
    );

    galaxies
}

fn expand_galaxies_along_one_dimension(galaxies: &mut Vec<Galaxy>,
                                       expansion_factor: usize,
                                       dimension: fn(&Galaxy) -> usize,
                                       galaxy_expander: fn(&mut Galaxy, usize)) {
    galaxies.sort_by(|g1: &Galaxy, g2: &Galaxy| dimension(g1).cmp(&dimension(g2)));

    let number_of_galaxies = galaxies.iter().count();
    for galaxy_index in 0..number_of_galaxies - 1 {
        let g1 = galaxies.get(galaxy_index).unwrap();
        let g2 = galaxies.get(galaxy_index + 1).unwrap();

        let galaxy_distance = dimension(g2) - dimension(g1);
        if galaxy_distance <= 1 {
            continue;
        }

        let expansion = (galaxy_distance - 1) * (expansion_factor - 1);

        for remaining_galaxy_index in galaxy_index + 1..number_of_galaxies {
            galaxy_expander(galaxies.get_mut(remaining_galaxy_index).unwrap(), expansion)
        }
    }
}

fn parse_galaxies(input: &str) -> Vec<Galaxy> {
    let rows = read_lines(input);
    let mut galaxy_index: usize = 1;
    let mut galaxies: Vec<Galaxy> = vec![];
    for (y, row) in rows.iter().enumerate() {
        for (x, c) in row.chars().enumerate() {
            if c == '#' {
                galaxies.push(Galaxy { index: galaxy_index, x, y });
                galaxy_index += 1;
            }
        }
    }
    galaxies
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::day11::*;
    use crate::input_reader::read_input_file;

    #[test]
    fn it_solves_puzzle() {
        let input = read_input_file("input_day11.txt");

        assert_eq!(9648398, calculate_sum_distances(expand_galaxies(&input, 2)));
        assert_eq!(618800410814, calculate_sum_distances(expand_galaxies(&input, 1000000)));
    }

    #[test]
    fn it_calculates_sum_distances() {
        let input = indoc! {"
        ...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#....."};

        assert_eq!(374, calculate_sum_distances(expand_galaxies(input, 2)));
        assert_eq!(1030, calculate_sum_distances(expand_galaxies(input, 10)));
        assert_eq!(8410, calculate_sum_distances(expand_galaxies(input, 100)));
    }
}