use itertools::Itertools;

// Adjacency list. If zeroth element is (7,2) then cup 1 is after cup 8 but before cup 3.
type Adjaceny = (usize, usize);
type AdjList = Vec<Adjaceny>;

fn link_cups(before: usize, after: usize, cups: &mut AdjList) {
    cups[before].1 = after;
    cups[after].0 = before;
}

fn parse_cups_part1(input: &str) -> AdjList {
    let labels = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize - 1)
        .collect_vec();
    let mut cups = vec![(0, 0); labels.len()];
    for (&a, &b) in labels.iter().tuple_windows() {
        link_cups(a, b, &mut cups)
    }
    let first = *labels.first().unwrap();
    let last = *labels.last().unwrap();
    link_cups(last, first, &mut cups);
    cups
}

fn shift_three_cups(current: usize, cups: &mut AdjList) {
    let front = cups[current].1;
    let middle = cups[front].1;
    let back = cups[middle].1;
    let after = cups[back].1;
    link_cups(current, after, cups);
    let destination = {
        let mut destination = current.checked_sub(1).unwrap_or(cups.len() - 1);
        while [front, middle, back].contains(&destination) {
            destination = destination.checked_sub(1).unwrap_or(cups.len() - 1);
        }
        destination
    };
    let after_destination = cups[destination].1;
    link_cups(destination, front, cups);
    link_cups(back, after_destination, cups);
}

fn print_cups(cups: &[Adjaceny]) -> String {
    let mut result = String::new();
    let mut next = cups[0].1;
    while next != 0 {
        result.push_str(&(next + 1).to_string());
        next = cups[next].1;
    }
    result
}

fn cup_game(mut cups: AdjList, start: usize, moves: usize) -> String {
    let mut curr = start;
    for _ in 0..moves {
        shift_three_cups(curr, &mut cups);
        curr = cups[curr].1;
    }
    print_cups(&cups)
}

/// Part 2
fn parse_cups_part2(input: &str) -> AdjList {
    let labels = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize - 1)
        .collect_vec();
    let mut cups = vec![(0, 0); labels.len()];
    for (&a, &b) in labels.iter().tuple_windows() {
        link_cups(a, b, &mut cups);
    }

    cups.extend((labels.len()..1_000_000).map(|n| (n - 1, n + 1)));
    link_cups(*labels.last().unwrap(), labels.len(), &mut cups);
    link_cups(cups.len() - 1, *labels.first().unwrap(), &mut cups);
    cups
}

fn cup_game_part2(mut cups: AdjList, start: usize, moves: usize) -> usize {
    let mut curr = start;
    for _ in 0..moves {
        shift_three_cups(curr, &mut cups);
        curr = cups[curr].1;
    }
    let a = cups[0].1;
    let b = cups[a].1;
    (a + 1) * (b + 1)
}

#[cfg(test)]
mod tests {
    use crate::day23::{cup_game, cup_game_part2, parse_cups_part1, parse_cups_part2};
    #[test]
    fn example1() {
        let input = "389125467";
        let cups = parse_cups_part1(input);
        let ans = cup_game(cups, 2, 100);
        assert_eq!(ans, "67384529");
    }

    #[test]
    fn part1() {
        let input = "327465189";
        let cups = parse_cups_part1(input);
        let ans = cup_game(cups, 2, 100);
        assert_eq!(ans, "82934675");
    }

    #[test]
    fn example2() {
        let input = "389125467";
        let cups = parse_cups_part2(input);
        let ans = cup_game_part2(cups, 2, 10_000_000);
        assert_eq!(ans, 149245887792);
    }

    #[test]
    fn part2() {
        let input = "327465189";
        let cups = parse_cups_part2(input);
        let ans = cup_game_part2(cups, 2, 10_000_000);
        assert_eq!(ans, 474600314018);
    }
}
