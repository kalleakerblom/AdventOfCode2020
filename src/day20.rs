struct Tile {
    id: usize,
    map: Vec<Vec<char>>,
}

impl Tile {
    fn parse(input: &str) -> Self {
        todo!()
    }
}


#[cfg(test)]
mod tests {
    use std::fs;
    #[test]
    fn example() {
        let input = fs::read_to_string("input/example").unwrap();
    }
    #[test]
    fn part1() {}
    #[test]
    fn part2() {}
}
