enum Ops {
    Add,
    Mul,
}
// TODO: Reuse part2 for part1, make binding powers a variable?
fn eval_part1(mut pos: usize, chars: &[char]) -> (usize, usize) {
    let mut res = 0;
    let mut op = Ops::Add;
    while pos < chars.len() {
        match chars[pos] {
            n @ '1'..='9' => {
                let n = n.to_digit(10).unwrap() as usize;
                match op {
                    Ops::Add => res += n,
                    Ops::Mul => res *= n,
                }
                pos += 1;
            }
            '(' => {
                let (new_pos, n) = eval_part1(pos + 1, chars);
                pos = new_pos;
                match op {
                    Ops::Add => res += n,
                    Ops::Mul => res *= n,
                }
            }
            '+' => {
                op = Ops::Add;
                pos += 1;
            }
            '*' => {
                op = Ops::Mul;
                pos += 1;
            }
            ')' => {
                pos += 1;
                break;
            }
            _ => pos += 1,
        }
    }

    (pos, res)
}

fn eval_part2(pos: &mut usize, chars: &[char], bind_pow: u8) -> usize {
    let mut lhs = match chars[*pos] {
        n @ '1'..='9' => {
            *pos += 1;
            n.to_digit(10).unwrap() as usize
        }
        '(' => {
            *pos += 1;
            let lhs = eval_part2(pos, chars, 0);
            *pos += 1;
            lhs
        }
        _ => panic!(),
    };
    while *pos < chars.len() {
        match chars[*pos] {
            '+' => {
                if bind_pow > 2 {
                    break;
                }
                *pos += 1;
                lhs += eval_part2(pos, chars, 2);
            }
            '*' => {
                if bind_pow > 1 {
                    break;
                }
                *pos += 1;
                lhs *= eval_part2(pos, chars, 1);
            }
            ')' => {
                break;
            }
            a => panic!("{}", a),
        }
    }
    lhs
}

fn do_part2(input: &str) -> usize {
    let characters: Vec<_> =
        input.chars().filter(|c| !c.is_whitespace()).collect();
    eval_part2(&mut 0, &characters, 0)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::{do_part2, eval_part1};
    #[test]
    fn example() {
        let input: Vec<_> = "1 + (2 * 3) + (4 * (5 + 6))".chars().collect();
        let ans = eval_part1(0, &input);
        assert_eq!(ans.1, 51);
        let input: Vec<_> =
            "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2".chars().collect();
        let ans = eval_part1(0, &input);
        assert_eq!(ans.1, 13632);
    }
    #[test]
    fn part1() {
        let input = fs::read_to_string("input/day18").unwrap();
        let ans: usize = input
            .lines()
            .map(|l| {
                let chars: Vec<_> = l.chars().collect();
                eval_part1(0, &chars).1
            })
            .sum();
        dbg!(ans);
    }
    #[test]
    fn example_part2() {
        let ans = do_part2("1 + 2 * 3 + 4 * 5 + 6");
        assert_eq!(ans, 231);
        let ans = do_part2("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2");
        assert_eq!(ans, 23340);
    }
    #[test]
    fn part2() {
        let input = fs::read_to_string("input/day18").unwrap();
        let ans: usize = input.lines().map(|l| do_part2(l)).sum();
        assert_eq!(ans, 119224703255966);
    }
}
