use scan_fmt::scan_fmt;

struct Password {
    min: usize,
    max: usize,
    c: char,
    pass: Vec<char>,
}

impl Password {
    fn verify(&self) -> bool {
        let count = self.pass.iter().filter(|&&c| self.c == c).count();
        self.min <= count && count <= self.max
    }
    fn verify2(&self) -> bool {
        (self.pass[self.min - 1] == self.c)
            ^ (self.pass[self.max - 1] == self.c)
    }
}
fn parse(input: &str) -> Vec<Password> {
    let mut result = Vec::new();
    for line in input.lines() {
        let (min, max, c, pass) =
            scan_fmt!(line, "{}-{} {}: {}", usize, usize, char, String)
                .unwrap();
        let pass = pass.chars().collect();
        result.push(Password { min, max, c, pass });
    }
    result
}

#[cfg(test)]
mod tests {
    use super::parse;
    use std::fs;
    #[test]
    fn example() {
        let inp = fs::read_to_string("input/example02").unwrap();
        let passwords = parse(&inp);
        let valid = passwords.iter().filter(|p| p.verify()).count();
        assert_eq!(valid, 2);
    }
    #[test]
    fn part1() {
        let inp = fs::read_to_string("input/day02").unwrap();
        let passwords = parse(&inp);
        let valid = passwords.iter().filter(|p| p.verify()).count();
        assert_eq!(valid, 600);
    }
    #[test]
    fn part2() {
        let inp = fs::read_to_string("input/day02").unwrap();
        let passwords = parse(&inp);
        let valid = passwords.iter().filter(|p| p.verify2()).count();
        assert_eq!(valid, 245);
    }
}
