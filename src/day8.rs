use std::collections::HashMap;

use regex::Regex;
use crate::input_reader::read_lines;

fn parse_node(line: &str) -> (String, (String, String)) {
    let node_regex = Regex::new(r"(.+) = \((.+), (.+)\)").unwrap();
    let capture = node_regex.captures(line).unwrap();
    let start = String::from(capture.get(1).unwrap().as_str());
    let left = String::from(capture.get(2).unwrap().as_str());
    let right = String::from(capture.get(3).unwrap().as_str());
    (start, (left, right))
}

fn parse_network(row_instructions: Vec<&String>) -> HashMap<String, (String, String)> {
    row_instructions.iter().map(|n| parse_node(n)).collect()
}

fn calculate_steps(instructions: &str, network: HashMap<String, (String, String)>) -> usize {
    let mut current_node = "AAA";
    let mut steps = 0;

    for instruction in instructions.chars().into_iter().cycle() {
        if current_node == "ZZZ" {
            return steps;
        }

        steps += 1;
        let (left, right) = &network[current_node];
        if instruction == 'L' {
            current_node = left.as_str();
        } else {
            current_node = right.as_str();
        }
    }

    panic!("ZZZ not found");
}

fn parse_input(input: &str) -> (String, HashMap<String, (String, String)>) {
    let lines = read_lines(&input);
    let instructions1 = lines.get(0).unwrap();
    let row_instructions = lines[2..].iter().clone().collect();
    let network1: HashMap<_, _> = parse_network(row_instructions);
    let (instructions, network) = (instructions1.clone(), network1);
    (instructions, network)
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::day8::*;
    use crate::input_reader::read_input_file;

    #[test]
    fn it_solves_first_part() {
        let input = read_input_file("input_day08.txt");

        let (instructions, network) = parse_input(&input);
        assert_eq!(13301, calculate_steps(&instructions, network));
    }

    #[test]
    fn it_calculates_steps() {
        let input = indoc! {"
        RL

        AAA = (BBB, CCC)
        BBB = (DDD, EEE)
        CCC = (ZZZ, GGG)
        DDD = (DDD, DDD)
        EEE = (EEE, EEE)
        GGG = (GGG, GGG)
        ZZZ = (ZZZ, ZZZ)"};


        let (instructions, network) = parse_input(input);
        assert_eq!(2, calculate_steps(&instructions, network));
    }
}