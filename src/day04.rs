use std::collections::HashMap;

const OPTIONAL: [&str; 1] = ["cid"];
const REQUIRED: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
const EYE_COLOR: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

fn valid_field(k: &str, v: &str) -> bool {
    let digits_in_range = |s, e, txt: &str| {
        let year: Option<u32> = txt.parse().ok();
        year.filter(|n| (s..=e).contains(n)).map(|_| true).unwrap_or(false)
    };
    match k {
        "byr" => digits_in_range(1920, 2002, v),
        "iyr" => digits_in_range(2010, 2020, v),
        "eyr" => digits_in_range(2020, 2030, v),
        "hgt" => {
            let (num, unit) = v.split_at(v.len() - 2);
            match unit {
                "cm" => digits_in_range(150, 193, num),
                "in" => digits_in_range(59, 76, num),
                _ => false,
            }
        }
        "hcl" => {
            let (a, b) = v.split_at(1);
            a == "#"
                && b.chars().all(|c| {
                    ('0'..='9').contains(&c) || ('a'..='f').contains(&c)
                })
        }
        "ecl" => EYE_COLOR.iter().any(|&color| color == v),
        "pid" => {
            v.chars().all(|c| c.is_ascii_digit());
            v.chars().count() == 9
        }
        _ => false,
    }
}
struct Passport(HashMap<String, String>);

impl Passport {
    fn is_valid(&self) -> bool {
        REQUIRED.iter().all(|&s| self.0.contains_key(s))
    }

    fn is_valid2(&self) -> bool {
        REQUIRED.iter().all(|&s| {
            self.0.contains_key(s) && valid_field(s, self.0.get(s).unwrap())
        })
    }
}

fn parse(input: &str) -> Vec<Passport> {
    let mut result = Vec::new();
    let mut pass: HashMap<String, String> = HashMap::new();
    for line in input.lines() {
        if line.is_empty() {
            result.push(Passport(pass.clone()));
            pass = HashMap::new();
        }
        let split = line.split_whitespace();
        for keyval in split {
            let kv_split: Vec<&str> = keyval.splitn(2, ':').collect();
            pass.insert(kv_split[0].to_string(), kv_split[1].to_string());
        }
    }
    if !pass.is_empty() {
        result.push(Passport(pass))
    }
    result
}
#[cfg(test)]
mod tests {
    use std::fs;

    use super::parse;
    #[test]
    fn example() {
        let inp = fs::read_to_string("input/example04").unwrap();
        let passports = parse(&inp);
        let valid = passports.iter().filter(|p| p.is_valid()).count();
        assert_eq!(valid, 2);
    }
    #[test]
    fn part1() {
        let inp = fs::read_to_string("input/day04").unwrap();
        let passports = parse(&inp);
        let valid = passports.iter().filter(|p| p.is_valid()).count();
        assert_eq!(valid, 204);
        // 203 too low;
    }
    #[test]
    fn part2() {
        let inp = fs::read_to_string("input/day04").unwrap();
        let passports = parse(&inp);
        let valid = passports.iter().filter(|p| p.is_valid2()).count();
        assert_eq!(valid, 179);
    }
}
