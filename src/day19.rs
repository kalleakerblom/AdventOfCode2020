use std::{
    collections::{HashMap, HashSet},
    iter,
};
#[derive(Debug)]
enum Rule {
    Match(String),
    Subrule(Vec<usize>),
    OrSubRules(Vec<usize>, Vec<usize>),
}

fn parse(input: &str) -> (HashMap<usize, Rule>, Vec<String>) {
    let (rules, msgs) = input.split_once("\r\n\r\n").unwrap();
    let scan_rule = |rule: &str| {
        rule.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect::<Vec<usize>>()
    };
    let rules = rules
        .lines()
        .map(|l| {
            let (id, rule_str) = l.split_once(": ").unwrap();
            let rule = if rule_str.contains('|') {
                let (rule1, rule2) = rule_str.split_once(" | ").unwrap();
                let r1 = scan_rule(rule1);
                let r2 = scan_rule(rule2);
                Rule::OrSubRules(r1, r2)
            } else if rule_str.contains('"') {
                Rule::Match(rule_str.trim_matches('"').to_string())
            } else {
                let r = scan_rule(rule_str);
                Rule::Subrule(r)
            };
            (id.parse().unwrap(), rule)
        })
        .collect();
    let msgs = msgs.lines().map(str::to_string).collect();

    (rules, msgs)
}
fn check_rule<'a>(
    s: &'a str,
    r: &Rule,
    rule_map: &HashMap<usize, Rule>,
) -> HashSet<&'a str> {
    match r {
        Rule::Match(m) => s
            .strip_prefix(m)
            .map(|rem| iter::once(rem).collect())
            .unwrap_or_else(HashSet::new),
        Rule::Subrule(subrules) => {
            subrules.iter().fold(iter::once(s).collect(), |acc, sub_id| {
                acc.iter()
                    .map(|s| check_rule(s, &rule_map[&sub_id], rule_map))
                    .fold_first(|sets, set| &sets | &set)
                    .unwrap_or_else(HashSet::new)
            })
        }
        Rule::OrSubRules(subrules1, subrules2) => {
            let res1: HashSet<_> = subrules1.iter().fold(
                iter::once(s).collect(),
                |acc, sub_id| {
                    acc.iter()
                        .map(|s| check_rule(s, &rule_map[&sub_id], rule_map))
                        .fold_first(|sets, set| &sets | &set)
                        .unwrap_or_else(HashSet::new)
                },
            );
            let res2: HashSet<_> = subrules2.iter().fold(
                iter::once(s).collect(),
                |acc, sub_id| {
                    acc.iter()
                        .map(|s| check_rule(s, &rule_map[&sub_id], rule_map))
                        .fold_first(|sets, set| &sets | &set)
                        .unwrap_or_else(HashSet::new)
                },
            );
            &res1 | &res2
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::{check_rule, parse, Rule};
    #[test]
    fn example() {
        let input = fs::read_to_string("input/example19").unwrap();
        let (rule_map, messages) = parse(&input);
        let ans = messages
            .iter()
            .map(|msg| check_rule(msg, &rule_map[&0], &rule_map))
            .filter(|res| res.contains(""))
            .count();
        assert_eq!(ans, 2);
    }
    #[test]
    fn part1() {
        let input = fs::read_to_string("input/day19").unwrap();
        let (rule_map, messages) = parse(&input);
        let ans = messages
            .iter()
            .map(|msg| check_rule(msg, &rule_map[&0], &rule_map))
            .filter(|res| res.contains(""))
            .count();
        assert_eq!(ans, 142);
    }

    #[test]
    fn example_part2() {
        let input = fs::read_to_string("input/example19_part2").unwrap();
        let (mut rule_map, messages) = parse(&input);
        rule_map.insert(8, Rule::OrSubRules(vec![42], vec![42, 8]));
        rule_map.insert(11, Rule::OrSubRules(vec![42, 31], vec![42, 11, 31]));
        let ans = messages
            .iter()
            .map(|msg| check_rule(msg, &rule_map[&0], &rule_map))
            .filter(|res| res.contains(""))
            .count();
        assert_eq!(ans, 12);
    }
    #[test]
    fn part2() {
        let input = fs::read_to_string("input/day19").unwrap();
        let (mut rule_map, messages) = parse(&input);
        rule_map.insert(8, Rule::OrSubRules(vec![42], vec![42, 8]));
        rule_map.insert(11, Rule::OrSubRules(vec![42, 31], vec![42, 11, 31]));
        let ans = messages
            .iter()
            .map(|msg| check_rule(msg, &rule_map[&0], &rule_map))
            .filter(|res| res.contains(""))
            .count();
        assert_eq!(ans, 294);
    }
}
