use std::collections::VecDeque;

fn add_num(deque: &mut VecDeque<usize>, num: usize) -> bool {
    for i in 0..deque.len() - 1 {
        for j in (i + 1)..deque.len() {
            if deque[i] + deque[j] == num {
                deque.push_back(num);
                deque.pop_front();
                return true;
            }
        }
    }
    false
}
fn xmas(preamble: &[&str], stream: &[&str]) -> usize {
    let mut deque: VecDeque<usize> =
        preamble.iter().map(|s| s.parse().unwrap()).collect();
    for n in stream.iter().map(|s| s.parse().unwrap()) {
        if !add_num(&mut deque, n) {
            return n;
        }
    }
    panic!()
}

fn xmas2(input: &str, goal: usize) -> (usize, usize) {
    let mut deque: VecDeque<usize> = VecDeque::new();
    let mut sum = 0;
    for n in input.lines().map(|s| s.parse().unwrap()) {
        while sum > goal {
            sum -= deque.pop_front().unwrap();
        }
        if sum == goal {
            let min = deque.iter().min().unwrap();
            let max = deque.iter().max().unwrap();
            return (*min, *max);
        }
        sum += n;
        deque.push_back(n);
    }
    panic!()
}
fn xmas2_v2(input: &str, goal: usize) -> (usize, usize) {
    let nums: Vec<usize> = input.lines().map(|l| l.parse().unwrap()).collect();
    let mut sum = 0;
    let mut start = 0;
    let mut end = 0;
    while end < nums.len() - 1 {
        while sum > goal {
            sum -= nums[start];
            start += 1;
        }
        if sum == goal {
            let slice = &nums[start..end];
            let min = slice.iter().min().unwrap();
            let max = slice.iter().max().unwrap();
            return (*min, *max);
        }
        sum += nums[end];
        end += 1;
    }
    panic!()
}
#[cfg(test)]
mod tests {
    use std::fs;

    use super::{xmas, xmas2, xmas2_v2};
    #[test]
    fn example() {
        let input = fs::read_to_string("input/example09").unwrap();
        let lines: Vec<_> = input.lines().collect();
        let ans = xmas(&lines[..5], &lines[5..]);
        assert_eq!(ans, 127);
    }
    #[test]
    fn part1() {
        let input = fs::read_to_string("input/day09").unwrap();
        let lines: Vec<_> = input.lines().collect();
        let ans = xmas(&lines[..25], &lines[25..]);
        assert_eq!(ans, 1504371145);
    }
    #[test]
    fn part2() {
        let input = fs::read_to_string("input/day09").unwrap();
        let ans = xmas2(&input, 1504371145);
        assert_eq!(ans.0 + ans.1, 183278487);
    }

    #[test]
    fn part2_v2() {
        let input = fs::read_to_string("input/day09").unwrap();
        let ans = xmas2_v2(&input, 1504371145);
        assert_eq!(ans.0 + ans.1, 183278487);
    }
}
