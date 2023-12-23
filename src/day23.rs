#[cfg(test)]
mod tests {
    use std::collections::{HashSet, LinkedList};
    use indoc::indoc;
    use crate::input_reader::{read_input_file, read_lines};

    #[test]
    fn it_solves_first_part() {
        let input = &read_input_file("input_day23.txt");

        let path_lengths = find_path_lengths(input);
        assert_eq!(2238, *path_lengths.last().unwrap());
    }

    #[test]
    fn it_calculates_path_lengths() {
        let input = indoc! {"
            #.#####################
            #.......#########...###
            #######.#########.#.###
            ###.....#.>.>.###.#.###
            ###v#####.#v#.###.#.###
            ###.>...#.#.#.....#...#
            ###v###.#.#.#########.#
            ###...#.#.#.......#...#
            #####.#.#.#######.#.###
            #.....#.#.#.......#...#
            #.#####.#.#.#########v#
            #.#...#...#...###...>.#
            #.#.#v#######v###.###v#
            #...#.>.#...>.>.#.###.#
            #####v#.#.###v#.#.###.#
            #.....#...#...#.#.#...#
            #.#########.###.#.#.###
            #...###...#...#...#.###
            ###.###.#.###v#####v###
            #...#...#.#.>.>.#.>.###
            #.###.###.#.###.#.#v###
            #.....###...###...#...#
            #####################.#"};

        let path_lengths = find_path_lengths(input);
        assert_eq!(vec![74, 82, 82, 86, 90, 94], path_lengths);
        assert_eq!(94, *path_lengths.last().unwrap());
    }

    fn find_path_lengths(input: &str) -> Vec<usize> {
        let map = read_lines(input).iter()
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let width = map[0].len();
        let length = map.len();
        let dimensions = (width, length);
        let target = (width - 2, length - 1);
        let start: (usize, usize) = (1, 0);

        let mut queue: LinkedList<((usize, usize), HashSet<(usize, usize)>)> = LinkedList::new();
        queue.push_front((start, HashSet::new()));
        let mut path_lengths: Vec<usize> = vec![];

        while let Some((node, path)) = queue.pop_front() {
            if node == target {
                path_lengths.push(path.len());
                continue;
            }
            let (x, y) = node;
            expand(
                ((x + 1) as isize, y as isize),
                '>',
                &map,
                &dimensions,
                &path,
                &mut queue,
            );

            expand(
                ((x as isize) - 1, y as isize),
                '<',
                &map,
                &dimensions,
                &path,
                &mut queue,
            );

            expand(
                (x as isize, (y + 1) as isize),
                'v',
                &map,
                &dimensions,
                &path,
                &mut queue,
            );

            expand(
                (x as isize, (y as isize) - 1),
                '^',
                &map,
                &dimensions,
                &path,
                &mut queue,
            );
        }

        path_lengths.sort();
        path_lengths
    }

    fn expand(
        neighbour: (isize, isize),
        slope: char,
        map: &Vec<Vec<char>>,
        (width, length): &(usize, usize),
        path: &HashSet<(usize, usize)>,
        queue: &mut LinkedList<((usize, usize), HashSet<(usize, usize)>)>,
    ) {
        if neighbour.0 < 0 || neighbour.0 >= (*width as isize) || neighbour.1 < 0 || neighbour.1 >= (*length as isize) {
            return;
        }

        let neighbour = (neighbour.0 as usize, neighbour.1 as usize);
        if path.contains(&neighbour) {
            return;
        }

        let tile = map[neighbour.1][neighbour.0];
        if tile == '.' {
            let mut new_path = path.clone();
            new_path.insert(neighbour);
            queue.push_front((neighbour, new_path));
            return;
        }

        if tile == slope {
            let mut new_path = path.clone();
            new_path.insert(neighbour);
            let neighbour = match slope {
                '^' => (neighbour.0, neighbour.1 - 1),
                'v' => (neighbour.0, neighbour.1 + 1),
                '>' => (neighbour.0 + 1, neighbour.1),
                '<' => (neighbour.0 - 1, neighbour.1),
                _ => panic!("unexpected slope")
            };
            new_path.insert(neighbour);
            queue.push_front((neighbour, new_path));
        }
    }
}