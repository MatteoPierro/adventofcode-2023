use std::collections::HashMap;
use num::integer::lcm;

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

fn calculate_steps(
    instructions: &str,
    network: HashMap<String, (String, String)>,
    select_start_nodes: fn(&str) -> bool,
    select_steps: fn(&str) -> bool,
) -> usize {
    network.keys()
        .filter(|node| select_start_nodes(node.as_str()))
        .map(|current_node|
            calculate_step_for_single_node(instructions, &network, select_steps, &mut current_node.clone())
        ).reduce(lcm)
        .unwrap()
}

fn calculate_step_for_single_node<'a>(
    instructions: &str,
    network: &'a HashMap<String, (String, String)>,
    select_steps: fn(&str) -> bool,
    mut current_node: &'a str,
) -> usize {
    let mut steps = 0;

    for instruction in instructions.chars().into_iter().cycle() {
        if select_steps(current_node) {
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

fn is_aaa_node(node: &str) -> bool {
    node == "AAA"
}


fn has_reached_zzz(node: &str) -> bool {
    node == "ZZZ"
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
        assert_eq!(13301, calculate_steps(&instructions, network, is_aaa_node, has_reached_zzz));
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
        assert_eq!(2, calculate_steps(&instructions, network, is_aaa_node, has_reached_zzz));
    }
}