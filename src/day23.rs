use std::collections::{HashMap, HashSet};
use crate::input_reader::read_lines;

struct Graph {
    map: Vec<Vec<char>>,
    width: usize,
    length: usize,
    start: (usize, usize),
    end: (usize, usize),
}

impl Graph {
    fn new(input: &str) -> Self {
        let map = read_lines(input).iter()
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let width = map[0].len();
        let length = map.len();

        Self { map, width, length, start: (1, 0), end: (width - 2, length - 1) }
    }

    fn find_point_of_interest(&self) -> Vec<(usize, usize)> {
        let mut pois = vec![];
        pois.push(self.start);
        pois.push(self.end);

        for (y, row) in self.map.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if *tile == '#' {
                    continue;
                }

                let mut neigbours = 0;
                for n in [(x as isize - 1, y as isize), ((x + 1) as isize, y as isize), (x as isize, y as isize - 1), (x as isize, (y + 1) as isize)] {
                    if self.is_in_range(n) && self.map[n.1 as usize][n.0 as usize] != '#' {
                        neigbours += 1;
                    }
                }

                if neigbours >= 3 {
                    pois.push((x, y))
                }
            }
        }

        pois
    }

    fn reduced_graph(&self) -> HashMap<(usize, usize), HashMap<(usize, usize), usize>> {
        let mut reduced_graph = HashMap::new();

        let pois = self.find_point_of_interest();
        for (start_x, start_y) in &pois {
            let mut stack = vec![(0, *start_x, *start_y)];
            let mut seen = HashSet::new();
            seen.insert((*start_x, *start_y));

            while !stack.is_empty() {
                let (steps, x, y) = stack.pop().unwrap();

                if pois.contains(&(x, y)) && (x, y) != (*start_x, *start_y) {
                    let e = reduced_graph.entry((*start_x, *start_y)).or_insert(HashMap::new());
                    e.insert((x, y), steps);
                    continue;
                }

                for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    let n = ((x as isize + dx), (y as isize + dy));
                    if self.is_in_range(n) && self.map[n.1 as usize][n.0 as usize] != '#' && !seen.contains(&(n.0 as usize, n.1 as usize)) {
                        stack.push((steps + 1, n.0 as usize, n.1 as usize));
                        seen.insert((n.0 as usize, n.1 as usize));
                    }
                }
            }
        }

        reduced_graph
    }

    fn is_in_range(&self, (x, y): (isize, isize)) -> bool {
        0 <= x && x < self.width as isize && 0 <= y && y < self.length as isize
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{HashSet, LinkedList};
    use indoc::indoc;
    use itertools::max;
    use crate::day23::Graph;
    use crate::input_reader::{read_input_file, read_lines};

    #[test]
    fn it_solves_first_part() {
        let input = &read_input_file("input_day23.txt");

        let path_lengths = find_path_lengths(input);
        assert_eq!(2238, *path_lengths.last().unwrap());
    }

    #[test]
    #[ignore] // really slow 47 seconds
    fn it_solves_second_part() {
        let input = &read_input_file("input_day23.txt");

        assert_eq!(6398, find_max_path_length_without_slopes(input));
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
        assert_eq!(154, find_max_path_length_without_slopes(input));
    }

    fn find_max_path_length_without_slopes(input: &str) -> usize {
        let graph = Graph::new(input);
        let reduced_graph = graph.reduced_graph();
        let mut queue: LinkedList<((usize, usize), Vec<(usize, usize)>, usize)> = LinkedList::new();
        queue.push_front((graph.start, vec![], 0));
        let mut path_lengths: Vec<usize> = vec![];

        while let Some((node, path, current_length)) = queue.pop_front() {
            if node == graph.end {
                path_lengths.push(current_length);
                continue;
            }

            for (n, &length) in &reduced_graph[&node] {
                if path.contains(n) {
                    continue
                }

                let mut new_path = path.clone();
                new_path.push(n.clone());
                queue.push_front((n.clone(), new_path, current_length + length));
            }
        }

        max(path_lengths).unwrap()
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