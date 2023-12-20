use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Pulse {
    Low,
    High,
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Modules {
    FlipFlop(bool, Vec<String>),
    Conjunction(HashMap<String, Pulse>, Vec<String>),
    Broadcaster(Vec<String>),
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, LinkedList};
    use indoc::indoc;
    use num::integer::lcm;
    use crate::day20::Modules::{Broadcaster, Conjunction, FlipFlop};
    use crate::day20::{Modules, Pulse};
    use crate::day20::Pulse::{High, Low};
    use crate::input_reader::{read_input_file, read_lines};

    #[test]
    fn it_solves_first_part() {
        let input = &read_input_file("input_day20.txt");

        let mut configuration = parse_configuration(input);
        let (low_pulses, high_pulses) = count_pulses(&mut configuration);
        assert_eq!(743871576, low_pulses * high_pulses);
    }

    #[test]
    fn it_solves_second_part() {
        let input = &read_input_file("input_day20.txt");

        // manual hack based on my input
        assert_eq!(3907, steps_to_turn_on_the_machine(&mut parse_configuration(input), "ph".to_string(), "kc".to_string()));
        assert_eq!(3797, steps_to_turn_on_the_machine(&mut parse_configuration(input), "vn".to_string(), "kc".to_string()));
        assert_eq!(4093, steps_to_turn_on_the_machine(&mut parse_configuration(input), "kt".to_string(), "kc".to_string()));
        assert_eq!(4021, steps_to_turn_on_the_machine(&mut parse_configuration(input), "hn".to_string(), "kc".to_string()));
        let values:[usize; 4] = [3907, 3797, 4093, 4021];
        let result= values.into_iter().reduce(|a, b| lcm(a, b)).unwrap();
        assert_eq!(244151741342687, result)
    }

    #[test]
    fn it_counts_pulses() {
        // let input: &str = indoc! {"
        // broadcaster -> a, b, c
        // %a -> b
        // %b -> c
        // %c -> inv
        // &inv -> a"};

        let input: &str = indoc! {"
        broadcaster -> a
        %a -> inv, con
        &inv -> b
        %b -> con
        &con -> output"};

        let mut configuration = parse_configuration(input);

        let (low_pulses, high_pulses) = count_pulses(&mut configuration);
        assert_eq!(11687500, low_pulses * high_pulses);
    }

    fn steps_to_turn_on_the_machine(configuration: &mut HashMap<String, Modules>, s: String, d: String) -> usize {
        let mut steps_to_turn_on_the_machine: usize = 0;

        loop {
            steps_to_turn_on_the_machine += 1;
            let mut sequence: LinkedList<(String, String, Pulse)> = LinkedList::new();
            sequence.push_back(("button".to_string(), "broadcaster".to_string(), Low));
            while let Some((source, destination, pulse)) = sequence.pop_front() {
                if source == s && destination == d && pulse == High {
                    return steps_to_turn_on_the_machine
                }

                if !configuration.contains_key(&destination) {
                    continue
                }
                match configuration.get_mut(&destination).unwrap() {
                    FlipFlop(state, outputs) => handle_flip_flop(&mut sequence, pulse, state, outputs, &destination),
                    Conjunction(inputs, outputs) => handling_conjunctions(&mut sequence, inputs, outputs, source, &destination, pulse),
                    Broadcaster(outputs) => handle_broadcaster(&mut sequence, outputs, pulse)
                }
            }
        }
    }

    fn count_pulses(configuration: &mut HashMap<String, Modules>) -> (usize, usize) {
        let mut low_pulses: usize = 0;
        let mut high_pulses: usize = 0;

        for _ in 0..1000 {
            let mut sequence: LinkedList<(String, String, Pulse)> = LinkedList::new();
            sequence.push_back(("button".to_string(), "broadcaster".to_string(), Low));
            while let Some((source, destination, pulse)) = sequence.pop_front() {
                if pulse == Low {
                    low_pulses += 1;
                } else {
                    high_pulses += 1;
                }

                if !configuration.contains_key(&destination) {
                    continue
                }
                match configuration.get_mut(&destination).unwrap() {
                    FlipFlop(state, outputs) => handle_flip_flop(&mut sequence, pulse, state, outputs, &destination),
                    Conjunction(inputs, outputs) => handling_conjunctions(&mut sequence, inputs, outputs, source, &destination, pulse),
                    Broadcaster(outputs) => handle_broadcaster(&mut sequence, outputs, pulse)
                }
            }
        }
        (low_pulses, high_pulses)
    }

    fn parse_configuration(input: &str) -> HashMap<String, Modules> {
        let mut configuration = HashMap::new();
        let mut inputs: HashMap<String, Vec<String>> = HashMap::new();
        let mut conjuctions: Vec<String> = vec![];

        for line in read_lines(input) {
            let parts = line.split(" -> ").collect::<Vec<_>>();
            let outputs = parts[1].split(", ").map(|s| s.to_string()).collect::<Vec<_>>();
            if parts[0] == "broadcaster" {
                for o in &outputs {
                    inputs.entry(o.to_string()).or_insert(vec![]);
                    inputs.get_mut(o).unwrap().push("broadcaster".to_string());
                }
                configuration.insert(parts[0].to_string(), Broadcaster(outputs));
                continue
            }

            let mut module = parts[0].to_string();

            match module.remove(0) {
                '%' => {
                    configuration.insert(module.clone(), FlipFlop(false, outputs.clone()));
                },
                '&' => {
                    configuration.insert(module.clone(), Conjunction(HashMap::new(), outputs.clone()));
                    conjuctions.push(module.clone());
                },
                _ => panic!("not expected")
            }

            for o in &outputs {
                inputs.entry(o.to_string()).or_insert(vec![]);
                inputs.get_mut(o).unwrap().push(module.clone());
            }
        }

        for c in &conjuctions {
            match configuration.get_mut(c).unwrap() {
                Conjunction(ci, _) => {
                    for i in inputs.get(c).unwrap() {
                        ci.insert(i.clone(), Low);
                    }
                },
                _ => panic!("not expected")
            }
        }
        configuration
    }

    fn handling_conjunctions(sequence: &mut LinkedList<(String, String, Pulse)>, inputs: &mut HashMap<String, Pulse>, outputs: &Vec<String>, source: String, conjunction_id: &String, pulse: Pulse) {
        inputs.insert(source, pulse);

        let pulse_to_send = if inputs.values().all(|&p| p == High) {
            Low
        } else {
            High
        };

        for o in outputs {
            sequence.push_back((conjunction_id.clone(), o.clone(), pulse_to_send))
        }
    }

    fn handle_flip_flop(sequence: &mut LinkedList<(String, String, Pulse)>, pulse: Pulse, state: &mut bool, outputs: &Vec<String>, flip_flop_id: &String) {
        if pulse == High {
            return;
        }

        let pulse_to_send = if *state {
            Low
        } else {
            High
        };

        *state = !*state;

        for o in outputs {
            sequence.push_back((flip_flop_id.clone(), o.clone(), pulse_to_send))
        }
    }

    fn handle_broadcaster(sequence: &mut LinkedList<(String, String, Pulse)>, outputs: &Vec<String>, pulse: Pulse) {
        for o in outputs {
            sequence.push_back(("broadcaster".to_string(), o.clone(), pulse))
        }
    }
}