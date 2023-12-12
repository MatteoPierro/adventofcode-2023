use std::collections::LinkedList;

use memoize::memoize;

use crate::input_reader::read_lines;

fn numbers_of_valid_springs(input: &str, expand: bool) -> Vec<usize> {
    read_lines(input).iter()
        .map(|s| {
            let parts: Vec<_> = s.split(" ").collect();
            let mut spring = parts[0].to_string();
            let mut size_continuous_damaged_groups: Vec<_> = parts[1].split(",").map(|s| s.parse::<usize>().unwrap()).collect();
            if expand {
                spring = std::iter::repeat(spring).take(5).collect::<Vec<_>>().join("?");
                size_continuous_damaged_groups = size_continuous_damaged_groups.iter().cycle()
                    .take(size_continuous_damaged_groups.len() * 5)
                    .cloned().
                    collect();
            }
            let spring1 = spring.chars().collect::<LinkedList<_>>();
            let groups_size = size_continuous_damaged_groups.iter().map(|i| *i).collect::<LinkedList<usize>>();
            calculate_number_of_valid_springs(spring1, groups_size, 0)
        }).collect()
}

#[memoize]
fn calculate_number_of_valid_springs(spring: LinkedList<char>, groups_size: LinkedList<usize>, damages_springs_counter: usize) -> usize {
    if spring.is_empty() && damages_springs_counter != 0 { // we saw damaged springs
        return if groups_size.len() == 1 && damages_springs_counter == *groups_size.front().unwrap() {
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

    let mut spring = spring;
    let next = spring.pop_front().unwrap();

    match next {
        '#' => calculate_number_of_valid_springs(spring, groups_size, damages_springs_counter + 1),

        '.' => { // arrived at the end of a chunk
            let mut groups_size = groups_size;
            if damages_springs_counter > 0 { // we saw damaged springs
                let expected_size = groups_size.pop_front().unwrap();
                if damages_springs_counter != expected_size {
                    return 0;
                }
            }
            calculate_number_of_valid_springs(spring, groups_size, 0)
        }

        _ => { // if it's a ?
            if groups_size.is_empty() { // we can continue checking if we find damaged springs or not
                return calculate_number_of_valid_springs(spring, groups_size, 0);
            }

            if *groups_size.front().unwrap() == damages_springs_counter { // we found a valid group of damaged springs let's move on
                let mut groups_size = groups_size;
                groups_size.pop_front();
                return calculate_number_of_valid_springs(spring, groups_size, 0);
            }

            if damages_springs_counter > 0 { // we are still counting a spring group. let's suppose that ? is a #.
                return calculate_number_of_valid_springs(spring, groups_size, damages_springs_counter + 1);
            }

            calculate_number_of_valid_springs(spring.clone(), groups_size.clone(), damages_springs_counter + 1)
                + calculate_number_of_valid_springs(spring.clone(), groups_size.clone(), damages_springs_counter)
        }
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::day12::*;
    use crate::input_reader::*;

    #[test]
    #[ignore] // completes in more than 16 seconds
    fn it_solves_first_part() {
        let input = read_input_file("input_day12.txt");

        assert_eq!(7251, numbers_of_valid_springs(&input, false).iter().sum::<usize>());
        assert_eq!(2128386729962, numbers_of_valid_springs(&input, true).iter().sum::<usize>());
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

        assert_eq!(vec![1, 4, 1, 1, 4, 10], numbers_of_valid_springs(input, false));
        assert_eq!(vec![1, 16384, 1, 16, 2500, 506250], numbers_of_valid_springs(input, true));
        assert_eq!(21, numbers_of_valid_springs(input, false).iter().sum::<usize>());
        assert_eq!(525152, numbers_of_valid_springs(input, true).iter().sum::<usize>());
    }
}