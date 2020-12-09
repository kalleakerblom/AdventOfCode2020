struct Seat {
    row: usize,
    col: usize,
}
impl Seat {
    // TODO: Replace for-loops with some bitshift magic
    fn parse(s: &str) -> Self {
        let mut chars = s.chars();
        let mut row = 0;
        let mut n = 64;
        for c in chars.by_ref().take(7) {
            match c {
                'F' => (),
                'B' => row += n,
                _ => panic!(),
            }
            n /= 2;
        }
        let mut col = 0;
        let mut n = 4;
        for c in chars.take(3) {
            match c {
                'L' => (),
                'R' => col += n,
                _ => panic!(),
            }
            n /= 2;
        }
        Seat { row, col }
    }
    fn get_id(&self) -> usize {
        self.row * 8 + self.col
    }
}

fn parse(input: &str) -> Vec<Seat> {
    input.lines().map(|l| Seat::parse(l)).collect()
}

fn find_missing_seat(seats: &[Seat]) -> Option<usize> {
    let mut ids_present = vec![false; 128 * 8];
    seats.iter().for_each(|s| ids_present[s.get_id()] = true);
    let triple_pos =
        ids_present.windows(3).position(|w| w == [true, false, true]);
    triple_pos.map(|tp| tp + 1)
}
#[cfg(test)]
mod tests {
    use std::fs;

    use super::{find_missing_seat, parse, Seat};
    #[test]
    fn example() {
        let inp = "FBFBBFFRLR";
        let s = Seat::parse(inp);
        assert_eq!(s.get_id(), 357);
    }
    #[test]
    fn part1() {
        let inp = fs::read_to_string("input/day05").unwrap();
        let seats = parse(&inp);
        let ans = seats.iter().map(|s| s.get_id()).max();
        assert_eq!(ans, Some(904));
    }
    #[test]
    fn part2() {
        let inp = fs::read_to_string("input/day05").unwrap();
        let seats = parse(&inp);
        let ans = find_missing_seat(&seats);
        assert_eq!(ans, Some(669));
    }
}
