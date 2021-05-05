const DIV: u64 = 20201227;
fn transform_loop(mut val: u64, subject: u64) -> u64 {
    val *= subject;
    val = val.rem_euclid(DIV);
    val
}

fn transform(subject: u64, loop_size: u64) -> u64 {
    let mut val = 1;
    for _ in 0..loop_size {
        val = transform_loop(val, subject);
    }
    val
}

fn find_loop_size(subject: u64, goal: u64) -> u64 {
    let mut val = 1;
    for i in 0.. {
        val = transform_loop(val, subject);
        if val == goal {
            return i + 1;
        }
    }
    unreachable!()
}
#[cfg(test)]
mod tests {
    use crate::day25::*;
    #[test]
    fn example() {
        let card_pub_key = 5764801;
        let door_pub_key = 17807724;
        let card = find_loop_size(7, card_pub_key);
        assert_eq!(card, 8);
        let door = find_loop_size(7, door_pub_key);
        assert_eq!(door, 11);
    }
    #[test]
    fn part1() {
        let card_pub_key = 11562782;
        let door_pub_key = 18108497;
        let card_loop_size = find_loop_size(7, card_pub_key);
        let ans = transform(door_pub_key, card_loop_size);
        assert_eq!(ans, 2947148);
    }
}
