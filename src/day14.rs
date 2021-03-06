use bitvec::prelude::*;
use scan_fmt::scan_fmt;
use std::collections::HashMap;

enum MaskBit {
    Floating,
    True,
    False,
}

fn parse_mask(input: &str) -> Vec<MaskBit> {
    input
        .chars()
        .map(|c| match c {
            'X' => MaskBit::Floating,
            '1' => MaskBit::True,
            '0' => MaskBit::False,
            _ => panic!("unknown symbol"),
        })
        .collect()
}

fn mask_number(n: usize, mask: &[MaskBit]) -> usize {
    let mut b = bitarr![Msb0, u64; 0; 36];
    b.store(n);
    for (pos, bit) in mask.iter().enumerate() {
        match bit {
            MaskBit::Floating => {}
            MaskBit::True => {
                b.set(pos + (64 - 36), true);
            }
            MaskBit::False => {
                b.set(pos + (64 - 36), false);
            }
        }
    }
    b.load()
}

fn do_part1(input: &str) -> usize {
    let mut memory: HashMap<usize, usize> = HashMap::new();
    let mut mask = vec![];
    for line in input.lines() {
        let (a, b) = line.split_once(" = ").unwrap();
        if a == "mask" {
            mask = parse_mask(b);
        } else {
            let i = scan_fmt!(a, "mem[{}]", usize).unwrap();
            let val = mask_number(b.parse().unwrap(), &mask);
            memory.insert(i, val);
        }
    }
    memory.values().sum()
}

fn do_part2(input: &str) -> usize {
    let mut memory: HashMap<usize, usize> = HashMap::new();
    let mut mask = vec![];
    for line in input.lines() {
        let (a, b) = line.split_once(" = ").unwrap();
        if a == "mask" {
            mask = parse_mask(b);
        } else {
            let i = scan_fmt!(a, "mem[{}]", usize).unwrap();
            let adresses = mask_adress(i, &mask);
            for i in adresses {
                memory.insert(i, b.parse().unwrap());
            }
        }
    }
    memory.values().sum()
}

fn mask_adress(n: usize, mask: &[MaskBit]) -> Vec<usize> {
    let mut bitarrays = vec![];
    let mut b = bitarr![Msb0, u64; 0; 36];
    b.store(n);
    bitarrays.push(b);
    for (pos, bit) in mask.iter().enumerate() {
        match bit {
            MaskBit::Floating => {
                bitarrays.extend(bitarrays.clone());
                let half = bitarrays.len() / 2;
                bitarrays
                    .iter_mut()
                    .take(half)
                    .for_each(|b| b.set(pos + (64 - 36), true));
                bitarrays
                    .iter_mut()
                    .skip(half)
                    .for_each(|b| b.set(pos + (64 - 36), false));
            }
            MaskBit::True => {
                bitarrays.iter_mut().for_each(|b| b.set(pos + (64 - 36), true))
            }
            MaskBit::False => {}
        }
    }
    bitarrays.iter().map(|b| b.load()).collect()
}
#[cfg(test)]
mod tests {
    use std::fs;

    use super::{do_part1, do_part2};
    #[test]
    fn example() {
        let input = fs::read_to_string("input/example14").unwrap();
        let ans = do_part1(&input);
        assert_eq!(ans, 165);
    }
    #[test]
    fn part1() {
        let input = fs::read_to_string("input/day14").unwrap();
        let ans = do_part1(&input);
        assert_eq!(ans, 9967721333886);
    }
    #[test]
    fn example2() {
        let input = fs::read_to_string("input/example14_part2").unwrap();
        let ans = do_part2(&input);
        assert_eq!(ans, 208);
    }
    #[test]
    fn part2() {
        let input = fs::read_to_string("input/day14").unwrap();
        let ans = do_part2(&input);
        assert_eq!(ans, 4355897790573);
    }
}
