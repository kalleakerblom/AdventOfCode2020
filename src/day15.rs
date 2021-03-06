use std::collections::HashMap;

fn do_part1(start_numbers: &[usize], end_turn: usize) -> usize {
    let mut memory: HashMap<usize, usize> =
        start_numbers.iter().enumerate().map(|(i, &n)| (n, i + 1)).collect();
    let mut last = 0;
    for turn in (start_numbers.len() + 1)..end_turn {
        let next = memory
            .insert(last, turn)
            .map(|prev_turn| turn - prev_turn)
            .unwrap_or(0);
        last = next;
    }
    last
}
#[cfg(test)]
mod tests {

    use super::do_part1;
    #[test]
    fn example() {
        let ans = do_part1(&[0, 3, 6], 2020);
        assert_eq!(ans, 436);
        let ans = do_part1(&[1, 3, 2], 2020);
        assert_eq!(ans, 1);
        let ans = do_part1(&[3, 1, 2], 2020);
        assert_eq!(ans, 1836);
    }
    #[test]
    fn part1() {
        let ans = do_part1(&[18, 11, 9, 0, 5, 1], 2020);
        assert_eq!(ans, 959);
    }
    #[test]
    fn part2() {
        // Part 1 works, but is slow!
        let ans = do_part1(&[18, 11, 9, 0, 5, 1], 30000000);
        assert_eq!(ans, 116590);
    }
}
