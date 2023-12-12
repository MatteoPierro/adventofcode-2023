use std::collections::LinkedList;

use crate::input_reader::read_lines;

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
    let mut spring = s.chars().collect::<LinkedList<_>>();
    let mut groups_size = size_continuous_damaged_groups.iter().map(|i| *i).collect::<LinkedList<usize>>();
    return calculate_number_of_damaged_springs(&mut spring, &mut groups_size, 0);
}

fn calculate_number_of_damaged_springs(spring: &mut LinkedList<char>, groups_size: &mut LinkedList<usize>, damages_springs_counter: usize) -> usize {
    if spring.is_empty() && damages_springs_counter != 0 { // we saw damaged springs
        return if groups_size.len() == 1 && damages_springs_counter == groups_size.pop_front().unwrap() {
            1
        } else {
            0
        };
    }

    if spring.is_empty() && damages_springs_counter == 0 { // we never saw damaged springs
        return if groups_size.is_empty() { // it wasn't supposed to have damaged springs
            1
        } else {
            0
        };
    }

    if damages_springs_counter > 0 && groups_size.is_empty() { // we saw more damaged spring than expected
        return 0;
    }

    if damages_springs_counter > 0 && damages_springs_counter > *groups_size.front().unwrap() { // we saw more damaged spring than expected
        return 0;
    }

    let next = spring.pop_front().unwrap();

    match next {
        '#' => calculate_number_of_damaged_springs(spring, groups_size, damages_springs_counter + 1),

        '.' => { // arrived at the end of a chunk
            if damages_springs_counter > 0 { // we saw damaged springs
                let expected_size = groups_size.pop_front().unwrap();
                if damages_springs_counter != expected_size {
                    return 0;
                }
            }
            calculate_number_of_damaged_springs(spring, groups_size, 0)
        }

        _ => {
            if groups_size.is_empty() { // we can continue checking if we find damaged springs or not
                return calculate_number_of_damaged_springs(spring, groups_size, 0);
            }

            if *groups_size.front().unwrap() == damages_springs_counter { // we found a valid group of damaged springs let's move on
                groups_size.pop_front();
                return calculate_number_of_damaged_springs(spring, groups_size, 0);
            }

            if damages_springs_counter > 0 { // we are still counting a spring group. let's suppose that ? is a #.
                return calculate_number_of_damaged_springs(spring, groups_size, damages_springs_counter + 1);
            }

            calculate_number_of_damaged_springs(&mut spring.clone(), &mut groups_size.clone(), damages_springs_counter + 1)
                + calculate_number_of_damaged_springs(&mut spring.clone(), &mut groups_size.clone(), damages_springs_counter)
        }
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::day12::*;
    use crate::input_reader::*;

    #[test]
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
    fn it_calculates_number_of_damaged_springs() {
        assert_eq!(1, calculate_number_of_damaged_springs(&mut "???.###".chars().collect::<LinkedList<_>>(), &mut vec![1, 1, 3].iter().map(|i| *i).collect::<LinkedList<usize>>(), 0));
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