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
    fn it_calculates_the_hash() {
        assert_eq!(30, calculate_hash("rn=1"));
    }

    #[test]
    fn it_calculates_initialization_sequence_hash_sum() {
        let row = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

        assert_eq!(1320, calculate_initialization_sequence_hash_sum(row));
    }
}