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
    let distances: Vec<_> = galaxies.into_iter().combinations(2).map(|combination| {
        let first = combination.first().unwrap();
        let second = combination.last().unwrap();
        first.distance(second)
    }).collect();

    distances.iter().sum::<usize>()
}

fn expand_galaxy(input: &str, expansion_factor: usize) -> Vec<Galaxy> {
    let mut galaxies = parse_galaxy(input);

    galaxies.sort_by(|g1, g2| g1.y.cmp(&g2.y));

    let mut index: usize = 0;
    let number_of_galaxies = galaxies.iter().count();
    while index < number_of_galaxies - 1 {
        let g1 = galaxies.get(index).unwrap();
        let g2 = galaxies.get(index + 1).unwrap();
        let mut expansion = g2.y - g1.y;
        if expansion <= 1 {
            index += 1;
            continue;
        }

        expansion = (expansion - 1) * (expansion_factor - 1);

        for j in index + 1..number_of_galaxies {
            galaxies.get_mut(j).unwrap().expand_row(expansion);
        }

        index += 1;
    }

    galaxies.sort_by(|g1, g2| g1.x.cmp(&g2.x));

    let mut index: usize = 0;
    let number_of_galaxies = galaxies.iter().count();
    while index < number_of_galaxies - 1 {
        let g1 = galaxies.get(index).unwrap();
        let g2 = galaxies.get(index + 1).unwrap();

        let mut expansion = g2.x - g1.x;
        if expansion <= 1 {
            index += 1;
            continue;
        }

        expansion = (expansion - 1) * (expansion_factor - 1);
        for j in index + 1..number_of_galaxies {
            galaxies.get_mut(j).unwrap().expand_column(expansion);
        }

        index += 1;
    }

    galaxies.sort_by(|g1, g2| g1.index.cmp(&g2.index));
    galaxies
}

fn parse_galaxy(input: &str) -> Vec<Galaxy> {
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
    fn it_solves_first_part() {
        let input = read_input_file("input_day11.txt");

        assert_eq!(9648398, calculate_sum_distances(expand_galaxy(&input, 2)));
        assert_eq!(618800410814, calculate_sum_distances(expand_galaxy(&input, 1000000)));
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

        assert_eq!(374, calculate_sum_distances(expand_galaxy(input, 2)));
        assert_eq!(1030, calculate_sum_distances(expand_galaxy(input, 10)));
        assert_eq!(8410, calculate_sum_distances(expand_galaxy(input, 100)));
    }
}