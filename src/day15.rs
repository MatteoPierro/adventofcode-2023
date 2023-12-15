use std::collections::HashMap;
use itertools::Itertools;

fn calculate_initialization_sequence_hash_sum(row: &str) -> usize {
    row.split(",").map(calculate_hash).sum::<usize>()
}

fn calculate_hash(step: &str) -> usize {
    let mut hash: usize = 0;

    for c in step.chars() {
        hash += c as usize;
        hash *= 17;
        hash %= 256;
    }
    hash
}

fn calculate_focusing_power(row: &str) -> usize {
    let raw_instructions = row.split(",").collect::<Vec<_>>();
    let mut boxes: HashMap<usize, Vec<&str>> = HashMap::new();
    let mut lens_value: HashMap<&str, usize> = HashMap::new();

    for instruction in raw_instructions {
        if instruction.contains('=') {
            let parts = instruction.split('=').collect::<Vec<_>>();
            let box_position = calculate_hash(parts[0]);
            let b = boxes.entry(box_position).or_insert(Vec::new());
            if !b.contains(&parts[0]) {
                b.push(&parts[0]);
            }
            let value = parts[1].parse::<usize>().unwrap();
            *lens_value.entry(&parts[0]).or_insert(value) = value;
        } else {
            let parts = instruction.split('-').collect::<Vec<_>>();
            let box_position = calculate_hash(parts[0]);
            let b = boxes.entry(box_position).or_insert(Vec::new());
            if let Some((index, _)) = b.iter().find_position(|&c| *c == parts[0]) {
                b.remove(index);
                lens_value.remove(&parts[0]);
            }
        }
    }

    let mut sum: usize = 0;
    for box_index in 0..256 {
        if boxes.get(&box_index) == None {
            continue;
        }

        for (len_slot, &len_name) in boxes.get(&box_index).unwrap().iter().enumerate() {
            sum += (box_index + 1) * (len_slot + 1) * lens_value.get(len_name).unwrap();
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use crate::day15::*;
    use crate::input_reader::{read_input_file, read_lines};

    #[test]
    fn it_solves_first_part() {
        let input = read_lines(&read_input_file("input_day15.txt"));

        assert_eq!(502139, calculate_initialization_sequence_hash_sum(&input[0]));
    }

    #[test]
    fn it_solves_second_part() {
        let input = read_lines(&read_input_file("input_day15.txt"));

        assert_eq!(284132, calculate_focusing_power(&input[0]));
    }

    #[test]
    fn it_calculates_focusing_power() {
        let row = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

        assert_eq!(145, calculate_focusing_power(row));
    }

    #[test]
    fn it_calculates_the_hash() {
        assert_eq!(30, calculate_hash("rn=1"));
    }

    #[test]
    fn it_calculates_initialization_sequence_hash_sum() {
        let row = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

        assert_eq!(1320, calculate_initialization_sequence_hash_sum(row));
    }
}