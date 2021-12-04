use std::collections::{HashMap, HashSet};

fn parse_bag_info(input: &str) -> (&str, Vec<(usize, &str, &str)>) {
    //"light red bags contain 1 bright white bag, 2 muted yellow bags.";
    let mut split = input.split(" bags contain ");
    let container = split.next().unwrap();
    let content = split.next().unwrap().strip_suffix('.').unwrap();
    if content == "no other bags" {
        return (container, vec![]);
    }
    let content: Vec<(usize, &str, &str)> = content
        .split(", ")
        .map(|s| {
            let mut split = s.split_whitespace();
            let n = split.next().unwrap().parse().unwrap();
            (n, split.next().unwrap(), split.next().unwrap())
        })
        .collect();
    (container, content)
}

type Rules = HashMap<String, Vec<(String, usize)>>;
fn parse_rules(input: &str) -> Rules {
    input
        .lines()
        .map(|line| parse_bag_info(line))
        .map(|info| {
            let key = info.0.to_string();
            let vals = info
                .1
                .iter()
                .map(|(n, s1, s2)| (s1.to_string() + " " + s2, *n))
                .collect();
            (key, vals)
        })
        .collect()
}

fn bag_contains_gold(bag: &str, memo: &mut HashSet<String>, rules: &Rules) -> bool {
    if memo.contains(bag) {
        return true;
    }
    for (lower_bag, _) in rules.get(bag).unwrap() {
        if bag_contains_gold(lower_bag, memo, rules) {
            memo.insert(bag.to_string());
            return true;
        }
    }
    false
}

fn bags_inside(bag: &str, memo: &mut HashMap<String, usize>, rules: &Rules) -> usize {
    if let Some(n) = memo.get(bag) {
        return *n;
    }
    let mut bag_count = 0;
    for (lower_bag, n) in rules.get(bag).unwrap() {
        bag_count += n * (1 + bags_inside(lower_bag, memo, rules));
    }
    memo.insert(bag.to_string(), bag_count);
    bag_count
}
#[cfg(test)]
mod tests {
    use std::{
        collections::{HashMap, HashSet},
        fs,
    };

    use super::{bag_contains_gold, bags_inside, parse_bag_info, parse_rules};
    #[test]
    fn example() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.";
        let parsed = parse_bag_info(input);
        dbg!(parsed);
    }
    #[test]
    fn part1() {
        let input = fs::read_to_string("input/day07").unwrap();
        let rules = parse_rules(&input);
        let mut memo = HashSet::new();
        memo.insert("shiny gold".to_string());
        let ans = rules
            .keys()
            .filter(|k| bag_contains_gold(k, &mut memo, &rules))
            .count();
        dbg!(ans);
    }
    #[test]
    fn part2() {
        let input = fs::read_to_string("input/day07").unwrap();
        let rules = parse_rules(&input);
        let mut memo = HashMap::new();
        let ans = bags_inside("shiny gold", &mut memo, &rules);
        dbg!(ans);
    }
}
