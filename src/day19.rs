#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use indoc::indoc;
    use regex::Regex;

    use crate::input_reader::{read_input_file, read_lines};

    #[test]
    fn it_solves_first_puzzle() {
        let input = &read_input_file("input_day19.txt");

        assert_eq!(476889, calculate_sum_accepted_ratings(input));
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
                    break
                }

                if result == "R" {
                    break
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
    }

    struct Workflow {
        rules: Vec<Box<dyn Rule>>
    }

    impl Workflow {
        fn new(raw_rules: &str) -> Self {
            let mut rules: Vec<Box<dyn Rule>> = vec![];

            for rule in raw_rules.split(","){
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
    }

    impl Rule for Workflow {
        fn execute(&self, ratings: &HashMap<String, usize>) -> Option<&str> {
            for rule in &self.rules {
                if let Some(result) = rule.execute(ratings) {
                    return Some(result)
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
    }
}