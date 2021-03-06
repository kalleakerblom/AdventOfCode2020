fn parse(input: &str) -> (usize, Vec<usize>) {
    let mut lines = input.lines();
    let earliest = lines.next().unwrap().parse().unwrap();
    let bus_ids = lines
        .next()
        .unwrap()
        .split(',')
        .filter(|&s| s != "x")
        .map(|s| s.parse().unwrap())
        .collect();

    (earliest, bus_ids)
}

fn find_wait_time_and_bus(
    earliest: usize,
    bus_ids: &[usize],
) -> (usize, usize) {
    bus_ids
        .iter()
        .map(|&id| {
            let wait = id - earliest % id;
            (wait, id)
        })
        .min()
        .unwrap()
}

fn parse2(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .nth(1)
        .unwrap()
        .split(',')
        .enumerate()
        .filter(|(_, s)| *s != "x")
        .map(|(i, s)| (i, s.parse().unwrap()))
        .collect()
}

fn do_part2(start: usize, step: usize, buses: &[(usize, usize)]) -> usize {
    for i in 0.. {
        let ans = start + i * step;
        if buses.iter().all(|(wait, id)| (ans + wait) % id == 0) {
            return ans;
        }
    }
    panic!("didn't find ans");
}
#[cfg(test)]
mod tests {
    use std::fs;

    use super::{do_part2, find_wait_time_and_bus, parse, parse2};
    #[test]
    fn example() {
        let (earliest, bus_ids) = parse("939\n7,13,x,x,59,x,31,19");
        let ans = find_wait_time_and_bus(earliest, &bus_ids);
        assert_eq!(ans.0 * ans.1, 295);
    }
    #[test]
    fn part1() {
        let input = fs::read_to_string("input/day13").unwrap();
        let (earliest, bus_ids) = parse(&input);
        let ans = find_wait_time_and_bus(earliest, &bus_ids);
        assert_eq!(ans.0 * ans.1, 153);
    }
    #[test]
    fn part2() {
        //let input = "0\n67,7,59,61";
        //let input = "0\n1789,37,47,1889";

        let input = fs::read_to_string("input/day13").unwrap();
        let buses = parse2(&input);
        let ans = do_part2(0, 1, &buses[..2]);
        assert_eq!(ans, 325);
        let step = &buses[..2].iter().map(|(_, id)| id).product();
        let ans = do_part2(325, *step, &buses[..4]);
        assert_eq!(ans, 11716731);
        let step = &buses[..4].iter().map(|(_, id)| id).product();
        let ans = do_part2(11716731, *step, &buses[..5]);
        assert_eq!(ans, 97272292);
        let step = &buses[..5].iter().map(|(_, id)| id).product();
        let ans = do_part2(97272292, *step, &buses);
        assert_eq!(ans, 471793476184394);
    }
}
