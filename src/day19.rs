#[cfg(test)]
mod tests {
    use std::cmp::{max, min};
    use std::collections::{HashMap, LinkedList};

    use indoc::indoc;
    use regex::Regex;

    use crate::input_reader::{read_input_file, read_lines};

    #[test]
    fn it_solves_first_part() {
        let input = &read_input_file("input_day19.txt");

        assert_eq!(476889, calculate_sum_accepted_ratings(input));
    }

    #[test]
    fn it_solves_second_part() {
        let input = &read_input_file("input_day19.txt");

        assert_eq!(132380153677887, calculate_distinct_combinations(input));
    }

    #[test]
    fn it_calculates_sum_accepted_ratings() {
        let input = indoc! {"
        px{a<2006:qkq,m>2090:A,rfg}
        pv{a>1716:R,A}
        lnx{m>1548:A,A}
        rfg{s<537:gd,x>2440:R,A}
        qs{s>3448:A,lnx}
        qkq{x<1416:A,crn}
        crn{x>2662:A,R}
        in{s<1351:px,qqz}
        qqz{s>2770:qs,m<1801:hdj,R}
        gd{a>3333:R,R}
        hdj{m>838:A,pv}

        {x=787,m=2655,a=1222,s=2876}
        {x=1679,m=44,a=2067,s=496}
        {x=2036,m=264,a=79,s=2244}
        {x=2461,m=1339,a=466,s=291}
        {x=2127,m=1623,a=2188,s=1013}"};

        assert_eq!(19114, calculate_sum_accepted_ratings(input));
    }

    #[test]
    fn it_calculates_distinct_combinations() {
        let input = indoc! {"
        px{a<2006:qkq,m>2090:A,rfg}
        pv{a>1716:R,A}
        lnx{m>1548:A,A}
        rfg{s<537:gd,x>2440:R,A}
        qs{s>3448:A,lnx}
        qkq{x<1416:A,crn}
        crn{x>2662:A,R}
        in{s<1351:px,qqz}
        qqz{s>2770:qs,m<1801:hdj,R}
        gd{a>3333:R,R}
        hdj{m>838:A,pv}

        {x=787,m=2655,a=1222,s=2876}
        {x=1679,m=44,a=2067,s=496}
        {x=2036,m=264,a=79,s=2244}
        {x=2461,m=1339,a=466,s=291}
        {x=2127,m=1623,a=2188,s=1013}"};

        assert_eq!(167409079868000, calculate_distinct_combinations(input));
    }

    fn calculate_distinct_combinations(input: &str) -> usize {
        let lines = read_lines(input);
        let parts = lines.split(|s| s.is_empty()).collect::<Vec<_>>();
        let raw_workflows = parts[0];
        let workflows = parse_workflows(raw_workflows);

        let mut accepted_rules: Vec<Vec<Box<dyn Rule>>> = vec![];
        let mut parts: LinkedList<(&str, Vec<Box<dyn Rule>>)> = LinkedList::new();
        parts.push_back(("in", vec![]));
        while !parts.is_empty() {
            let (key, rules) = parts.pop_front().unwrap();
            let workflow = &workflows[key];
            let mut rejected_rules: Vec<Box<dyn Rule>> = vec![];

            for rule_index in 0..workflow.rules.len() - 1 {
                let rule = &workflow.rules[rule_index];
                let mut new_rules: Vec<Box<dyn Rule>> = vec![];
                for r in &rules {
                    new_rules.push(r.copy());
                }

                for r in &rejected_rules {
                    new_rules.push(r.copy());
                }

                new_rules.push(rule.copy());
                rejected_rules.push(rule.opposite());

                match rule.result() {
                    "A" => accepted_rules.push(new_rules),
                    "R" => {}
                    _ => parts.push_back((rule.result(), new_rules))
                }
            }

            let mut new_rules: Vec<Box<dyn Rule>> = vec![];
            for r in &rules {
                new_rules.push(r.copy());
            }
            for r in &rejected_rules {
                new_rules.push(r.copy());
            }

            let result = workflow.rules.last().unwrap().result();
            match result {
                "A" => accepted_rules.push(new_rules),
                "R" => {}
                _ => parts.push_back((result, new_rules))
            }
        }

        accepted_rules.iter().map(count_combination_for_instruction).sum()
    }

    fn count_combination_for_instruction(instructions: &Vec<Box<dyn Rule>>) -> usize {
        let mut prod: usize = 1;
        for rating in "xmas".chars() {
            let mut rating_min: usize = 1;
            let mut rating_max: usize = 4000;

            for instruction in instructions {
                if instruction.key() != rating.to_string() {
                    continue;
                }

                instruction.update(&mut rating_max, &mut rating_min);
            }

            prod *= (rating_max - rating_min) + 1
        }

        prod
    }

    fn calculate_sum_accepted_ratings(input: &str) -> usize {
        let lines = read_lines(input);
        let parts = lines.split(|s| s.is_empty()).collect::<Vec<_>>();
        let raw_workflows = parts[0];
        let raw_ratings = parts[1];

        let all_ratings = raw_ratings.iter().map(|r| parse_rating(r))
            .collect::<Vec<_>>();

        let workflows = parse_workflows(raw_workflows);

        let mut sum = 0;

        for ratings in &all_ratings {
            let mut current_workflow = &workflows["in"];
            loop {
                let result = current_workflow.execute(&ratings).unwrap();
                if result == "A" {
                    sum += ratings.values().sum::<usize>();
                    break;
                }

                if result == "R" {
                    break;
                }

                current_workflow = &workflows[result];
            }
        }

        sum
    }

    fn parse_rating(raw_rating: &str) -> HashMap<String, usize> {
        let ratings_regex = Regex::new(r"(\w)=(\d+)").unwrap();

        let ratings = ratings_regex.captures_iter(raw_rating).map(|c| {
            let (_, [key, value]) = c.extract();
            (key.to_string(), value.parse::<usize>().unwrap())
        }).collect::<HashMap<_, _>>();
        ratings
    }

    fn parse_workflows(raw_workflows: &[String]) -> HashMap<&str, Workflow> {
        let workflow_regex = Regex::new(r"(.+)\{(.+)}").unwrap();
        let workflows = raw_workflows.iter().map(|raw_workflow| {
            let captures: [&str; 2] = workflow_regex.captures(&raw_workflow).map(|c| c.extract()).unwrap().1;
            (captures[0], Workflow::new(captures[1]))
        }).collect::<HashMap<_, _>>();
        workflows
    }

    trait Rule {
        fn execute(&self, ratings: &HashMap<String, usize>) -> Option<&str>;
        fn opposite(&self) -> Box<dyn Rule>;
        fn result(&self) -> &str;
        fn key(&self) -> &str;

        fn update(&self, rating_max: &mut usize, rating_min: &mut usize);

        fn copy(&self) -> Box<dyn Rule>;
    }

    struct Workflow {
        rules: Vec<Box<dyn Rule>>,
    }

    impl Workflow {
        fn new(raw_rules: &str) -> Self {
            let mut rules: Vec<Box<dyn Rule>> = vec![];

            for rule in raw_rules.split(",") {
                if rule.contains("<") {
                    rules.push(LowerRule::new(rule))
                } else if rule.contains(">") {
                    rules.push(GreaterRule::new(rule))
                } else {
                    rules.push(DefaultRule::new(rule))
                }
            }

            Workflow { rules }
        }

        fn execute(&self, ratings: &HashMap<String, usize>) -> Option<&str> {
            for rule in &self.rules {
                if let Some(result) = rule.execute(ratings) {
                    return Some(result);
                }
            }

            panic!("No rule matches!")
        }
    }

    #[derive(Debug)]
    struct LowerRule {
        key: String,
        value: usize,
        result: String,
    }

    impl LowerRule {
        fn new(raw_rule: &str) -> Box<Self> {
            let rule_parts = raw_rule.split(":").collect::<Vec<_>>();
            let raw_activation = rule_parts[0];
            let result = rule_parts[1];
            let activation_parts = raw_activation.split("<").collect::<Vec<_>>();
            let key = activation_parts[0];
            let value = activation_parts[1].parse::<usize>().unwrap();
            Box::new(LowerRule { key: key.to_string(), result: result.to_string(), value })
        }
    }

    impl Rule for LowerRule {
        fn execute(&self, ratings: &HashMap<String, usize>) -> Option<&str> {
            if *(ratings.get(&self.key).unwrap()) < self.value {
                Some(&self.result)
            } else {
                None
            }
        }

        fn opposite(&self) -> Box<dyn Rule> {
            Box::new(GreaterEqualRule { value: self.value, key: self.key.clone(), result: self.result.clone() })
        }

        fn result(&self) -> &str {
            &self.result
        }

        fn key(&self) -> &str {
            &self.key
        }

        fn update(&self, rating_max: &mut usize, _rating_min: &mut usize) {
            *rating_max = min(*rating_max, self.value - 1)
        }

        fn copy(&self) -> Box<dyn Rule> {
            Box::new(Self { value: self.value, result: self.result.clone(), key: self.key.clone() })
        }
    }

    #[derive(Debug)]
    struct LowerEqualRule {
        key: String,
        value: usize,
        result: String,
    }

    impl Rule for LowerEqualRule {
        fn execute(&self, ratings: &HashMap<String, usize>) -> Option<&str> {
            if *(ratings.get(&self.key).unwrap()) <= self.value {
                Some(&self.result)
            } else {
                None
            }
        }

        fn opposite(&self) -> Box<dyn Rule> {
            Box::new(GreaterRule { value: self.value, key: self.key.clone(), result: self.result.clone() })
        }

        fn result(&self) -> &str {
            &self.result
        }

        fn key(&self) -> &str {
            &self.key
        }

        fn update(&self, rating_max: &mut usize, _rating_min: &mut usize) {
            *rating_max = min(*rating_max, self.value)
        }

        fn copy(&self) -> Box<dyn Rule> {
            Box::new(Self { value: self.value, result: self.result.clone(), key: self.key.clone() })
        }
    }

    #[derive(Debug)]
    struct GreaterRule {
        key: String,
        value: usize,
        result: String,
    }

    impl GreaterRule {
        fn new(raw_rule: &str) -> Box<Self> {
            let rule_parts = raw_rule.split(":").collect::<Vec<_>>();
            let raw_activation = rule_parts[0];
            let result = rule_parts[1];
            let activation_parts = raw_activation.split(">").collect::<Vec<_>>();
            let key = activation_parts[0];
            let value = activation_parts[1].parse::<usize>().unwrap();
            Box::new(GreaterRule { key: key.to_string(), result: result.to_string(), value })
        }
    }

    impl Rule for GreaterRule {
        fn execute(&self, ratings: &HashMap<String, usize>) -> Option<&str> {
            if *(ratings.get(&self.key).unwrap()) > self.value {
                Some(&self.result)
            } else {
                None
            }
        }

        fn opposite(&self) -> Box<dyn Rule> {
            Box::new(LowerEqualRule { value: self.value, key: self.key.clone(), result: self.result.clone() })
        }

        fn result(&self) -> &str {
            &self.result
        }

        fn key(&self) -> &str {
            &self.key
        }

        fn update(&self, _rating_max: &mut usize, rating_min: &mut usize) {
            *rating_min = max(*rating_min, self.value + 1)
        }


        fn copy(&self) -> Box<dyn Rule> {
            Box::new(Self { value: self.value, result: self.result.clone(), key: self.key.clone() })
        }
    }

    #[derive(Debug)]
    struct GreaterEqualRule {
        key: String,
        value: usize,
        result: String,
    }

    impl Rule for GreaterEqualRule {
        fn execute(&self, ratings: &HashMap<String, usize>) -> Option<&str> {
            if *(ratings.get(&self.key).unwrap()) >= self.value {
                Some(&self.result)
            } else {
                None
            }
        }

        fn opposite(&self) -> Box<dyn Rule> {
            Box::new(LowerRule { value: self.value, key: self.key.clone(), result: self.result.clone() })
        }

        fn result(&self) -> &str {
            &self.result
        }

        fn key(&self) -> &str {
            &self.key
        }

        fn update(&self, _rating_max: &mut usize, rating_min: &mut usize) {
            *rating_min = max(*rating_min, self.value)
        }

        fn copy(&self) -> Box<dyn Rule> {
            Box::new(Self { value: self.value, result: self.result.clone(), key: self.key.clone() })
        }
    }

    #[derive(Debug)]
    struct DefaultRule {
        result: String,
    }

    impl DefaultRule {
        fn new(result: &str) -> Box<Self> {
            Box::new(DefaultRule { result: result.to_string() })
        }
    }

    impl Rule for DefaultRule {
        fn execute(&self, _ratings: &HashMap<String, usize>) -> Option<&str> {
            Some(&self.result)
        }

        fn opposite(&self) -> Box<dyn Rule> {
            panic!("no possible!")
        }

        fn result(&self) -> &str {
            &self.result
        }

        fn key(&self) -> &str {
            panic!("no key!")
        }

        fn update(&self, _rating_max: &mut usize, _rating_min: &mut usize) {
            panic!("not allowed")
        }

        fn copy(&self) -> Box<dyn Rule> {
            Box::new(Self { result: self.result.clone() })
        }
    }
}