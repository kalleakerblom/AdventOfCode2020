use std::collections::HashSet;

fn parse_forms(input: &str) -> Vec<HashSet<char>> {
    let mut result = Vec::new();
    let mut form = HashSet::new();
    for line in input.lines() {
        if line.is_empty() {
            result.push(form.clone());
            form = HashSet::new();
        }
        line.chars().for_each(|c| {
            form.insert(c);
        })
    }
    if !form.is_empty() {
        result.push(form)
    }
    result
}

fn parse_forms2(input: &str) -> Vec<Vec<&str>> {
    let lines: Vec<_> = input.lines().collect();
    lines.split(|l| l.is_empty()).map(|form| form.to_vec()).collect()
}

fn to_bitset(input: &str) -> usize {
    let mut bitset = 0;
    input.chars().for_each(|c| {
        let pos = c as u8 - b'a';
        bitset |= 1 << pos;
    });
    bitset
}

fn count_answers(forms: Vec<Vec<&str>>) -> u32 {
    let mut ans = 0;
    for form in forms {
        let (first, rest) = form.split_first().unwrap();
        let set =
            rest.iter().fold(to_bitset(first), |acc, s| acc & to_bitset(s));
        ans += set.count_ones();
    }
    ans
}
#[cfg(test)]
mod tests {
    use std::fs;

    use super::{count_answers, parse_forms, parse_forms2};
    #[test]
    fn example() {
        let input = fs::read_to_string("input/example06").unwrap();
        let forms = parse_forms(&input);
        let ans: usize = forms.iter().map(|f| f.len()).sum();
        assert_eq!(ans, 11);
    }
    #[test]
    fn part1() {
        let input = fs::read_to_string("input/day06").unwrap();
        let forms = parse_forms(&input);
        let ans: usize = forms.iter().map(|f| f.len()).sum();
        assert_eq!(ans, 6534);
    }
    #[test]
    fn example2() {
        let input = fs::read_to_string("input/example06").unwrap();
        let forms = parse_forms2(&input);
        let ans = count_answers(forms);
        assert_eq!(ans, 6);
    }
    #[test]
    fn part2() {
        let input = fs::read_to_string("input/day06").unwrap();
        let forms = parse_forms2(&input);
        let ans = count_answers(forms);
        assert_eq!(ans, 3402);
    }
}
