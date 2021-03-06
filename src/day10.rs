use std::{
    collections::{HashMap, HashSet},
    iter,
};

fn do_part1(input: &str) -> (u32, u32) {
    let mut adapters: Vec<u32> =
        input.lines().map(|l| l.parse().unwrap()).collect();
    adapters.push(0);
    adapters.sort_unstable();
    let mut plus_one = 0;
    let mut plus_three = 0;
    adapters.windows(2).map(|w| w[1] - w[0]).for_each(|step| match step {
        1 => plus_one += 1,
        3 => plus_three += 1,
        step => println!("{}", step),
    });
    (plus_one, plus_three + 1)
}

fn ways_to_reach_adapter(
    to_reach: usize,
    available: &std::collections::HashSet<usize>,
    memo: &mut HashMap<usize, usize>,
) -> usize {
    if let Some(ans) = memo.get(&to_reach) {
        return *ans;
    }
    if to_reach == 0 {
        return 1;
    }
    let mut ans = 0;
    if to_reach >= 1 && available.contains(&(to_reach - 1)) {
        ans += ways_to_reach_adapter(to_reach - 1, available, memo);
    }
    if to_reach >= 2 && available.contains(&(to_reach - 2)) {
        ans += ways_to_reach_adapter(to_reach - 2, available, memo);
    }
    if to_reach >= 3 && available.contains(&(to_reach - 3)) {
        ans += ways_to_reach_adapter(to_reach - 3, available, memo);
    }
    memo.insert(to_reach, ans);
    ans
}
fn do_part2(input: &str) -> usize {
    // TODO: Dense set, use some bitvec or Vec<bool>
    let available: HashSet<usize> = input
        .lines()
        .map(|l| l.parse().unwrap())
        .chain(iter::once(0))
        .collect();
    let max = available.iter().max().unwrap();
    let mut memo = HashMap::new();
    ways_to_reach_adapter(*max, &available, &mut memo)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::{do_part1, do_part2};
    #[test]
    fn example() {
        let input = fs::read_to_string("input/example10").unwrap();
        let ans = do_part1(&input);
        assert_eq!(ans, (22, 10));
    }
    #[test]
    fn part1() {
        let input = fs::read_to_string("input/day10").unwrap();
        let ans = do_part1(&input);
        assert_eq!(ans, (70, 27));
    }
    #[test]
    fn example2() {
        let input = fs::read_to_string("input/example10").unwrap();
        let ans = do_part2(&input);
        dbg!(ans, 19208);
    }
    #[test]
    fn part2() {
        let input = fs::read_to_string("input/day10").unwrap();
        let ans = do_part2(&input);
        assert_eq!(ans, 49607173328384);
    }
}
