use std::collections::LinkedList;

use crate::input_reader::read_lines;

fn expand_spring(s: &str) -> Vec<String> {
    let mut to_expand: LinkedList<String> = LinkedList::from([s.to_string()]);
    let mut expanded: Vec<String> = vec![];

    while !to_expand.is_empty() {
        let s = to_expand.pop_back().unwrap();
        if !s.contains("?") {
            expanded.push(s);
            continue;
        }

        to_expand.push_back(s.replacen("?", ".", 1).to_string());
        to_expand.push_back(s.replacen("?", "#", 1).to_string());
    }
    expanded
}

fn is_valid_spring(s: &str, size_continuous_damaged_groups: &Vec<usize>) -> bool {
    let pairs = s.split('.').filter(|&s| !s.is_empty()).map(|l| l.len()).collect::<Vec<_>>();
    // print!("{:?}", pairs);
    if pairs.len() != size_continuous_damaged_groups.len() {
        return false;
    }
    pairs.iter()
        .zip(size_continuous_damaged_groups)
        .all(|(&a, &b)| a == b)
}

fn numbers_of_valid_springs(input: &str) -> Vec<usize> {
    read_lines(input).iter()
        .map(|s| {
            let parts: Vec<_> = s.split(" ").collect();
            let spring = parts[0].to_string();
            let size_continuous_damaged_groups: Vec<_> = parts[1].split(",").map(|s| s.parse::<usize>().unwrap()).collect();
            // println!("{:?}", spring);
            // println!("{:?}", size_continuous_damaged_groups);
            number_of_valid_springs(&spring, &size_continuous_damaged_groups)
        }).collect()
}

fn number_of_valid_springs(s: &str, size_continuous_damaged_groups: &Vec<usize>) -> usize {
    let expanded = expand_spring(s);

    // println!("{:?}", expanded);
    // println!();

    expanded.iter().filter(|s| is_valid_spring(s, &size_continuous_damaged_groups)).count()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::day12::*;
    use crate::input_reader::*;

    #[test]
    #[ignore] // it's slow. runs in 14 seconds
    fn it_solves_first_part() {
        let input = read_input_file("input_day12.txt");

        assert_eq!(7251, numbers_of_valid_springs(&input).iter().sum::<usize>());
    }

    #[test]
    fn it_returns_sum_of_valid_springs() {
        let input = indoc! {"
        ???.### 1,1,3
        .??..??...?##. 1,1,3
        ?#?#?#?#?#?#?#? 1,3,1,6
        ????.#...#... 4,1,1
        ????.######..#####. 1,6,5
        ?###???????? 3,2,1"};

        assert_eq!(vec![1, 4, 1, 1, 4, 10], numbers_of_valid_springs(input));
        assert_eq!(21, numbers_of_valid_springs(input).iter().sum::<usize>());
    }

    #[test]
    fn it_calculates_the_number_of_valid_springs() {
        assert_eq!(1, number_of_valid_springs("???.###", &vec![1, 1, 3]));
        assert_eq!(4, number_of_valid_springs(".??..??...?##.", &vec![1, 1, 3]));
        assert_eq!(1, number_of_valid_springs("?#?#?#?#?#?#?#?", &vec![1, 3, 1, 6]));
        assert_eq!(1, number_of_valid_springs("????.#...#...", &vec![4, 1, 1]));
        assert_eq!(4, number_of_valid_springs("????.######..#####.", &vec![1, 6, 5]));
        assert_eq!(10, number_of_valid_springs("?###????????", &vec![3, 2, 1]));
    }
}