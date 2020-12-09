use std::collections::HashSet;

fn find_sum_part1(sum: u32, terms: &[u32]) -> Option<(u32, u32)> {
    let set: HashSet<u32> = terms.iter().cloned().collect();
    for &first in terms {
        let remainder = sum - first;
        if set.contains(&remainder) {
            return Some((first, remainder));
        }
    }
    None
}

fn find_sum_part2(sum: u32, mut terms: Vec<u32>) -> Option<(u32, u32, u32)> {
    terms.sort_unstable();
    for (i, &first) in terms.iter().enumerate() {
        if first >= sum {
            break;
        }
        let rem1 = sum - first;
        for (j, &second) in terms[i + 1..].iter().enumerate() {
            if second >= rem1 {
                break;
            }
            let rem2 = rem1 - second;
            for &third in &terms[i + j + 1..] {
                if third == rem2 {
                    return Some((first, second, third));
                }
                if third > rem2 {
                    break;
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::{find_sum_part1, find_sum_part2};

    #[test]
    fn example() {
        let inp = fs::read_to_string("input/example01").unwrap();
        let terms: Vec<_> =
            inp.lines().into_iter().map(|s| s.parse().unwrap()).collect();
        let res = find_sum_part1(2020, &terms);
        assert_eq!(res, Some((1721, 299)));
    }
    #[test]
    fn part1() {
        let inp = fs::read_to_string("input/day01").unwrap();
        let terms: Vec<_> =
            inp.lines().into_iter().map(|s| s.parse().unwrap()).collect();
        let res = find_sum_part1(2020, &terms);
        if let Some((a, b)) = res {
            assert_eq!(a * b, 786811);
        } else {
            panic!()
        }
    }
    #[test]
    fn part2() {
        let inp = fs::read_to_string("input/day01").unwrap();
        let terms =
            inp.lines().into_iter().map(|s| s.parse().unwrap()).collect();
        let res = find_sum_part2(2020, terms);
        if let Some((a, b, c)) = res {
            assert_eq!(a * b * c, 199068980);
        } else {
            panic!()
        }
    }
}
