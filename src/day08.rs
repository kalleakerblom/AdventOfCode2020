#[derive(Clone)]
enum Op {
    Jmp(i32),
    Acc(i32),
    Nop(i32),
}

#[derive(Clone)]
struct CPU {
    instructions: Vec<Op>,
    head: usize,
    acc: i32,
}

impl CPU {
    fn new(input: &str) -> Self {
        let instructions = input
            .lines()
            .map(|s| {
                let mut split = s.split_whitespace();
                match split.next().unwrap() {
                    "jmp" => Op::Jmp(split.next().unwrap().parse().unwrap()),
                    "acc" => Op::Acc(split.next().unwrap().parse().unwrap()),
                    "nop" => Op::Nop(split.next().unwrap().parse().unwrap()),
                    _ => panic!(),
                }
            })
            .collect();
        Self { instructions, head: 0, acc: 0 }
    }
    fn run(&mut self) -> bool {
        match self.instructions.get(self.head) {
            Some(Op::Jmp(i)) => {
                self.head = (self.head as i32 + i) as usize;
                true
            }
            Some(Op::Acc(i)) => {
                self.acc += i;
                self.head += 1;
                true
            }
            Some(Op::Nop(_)) => {
                self.head += 1;
                true
            }
            None => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashSet, fs};

    use super::{Op, CPU};
    #[test]
    fn example() {
        let input = fs::read_to_string("input/example08").unwrap();
        let mut cpu = CPU::new(&input);
        let mut ins_done = HashSet::new();
        ins_done.insert(0);
        while cpu.run() {
            let head = cpu.head;
            if ins_done.contains(&head) {
                break;
            } else {
                ins_done.insert(head);
            }
        }
        dbg!(cpu.acc);
    }
    #[test]
    fn part1() {
        let input = fs::read_to_string("input/day08").unwrap();
        let mut cpu = CPU::new(&input);
        let mut ins_done = HashSet::new();
        ins_done.insert(0);
        while cpu.run() {
            let head = cpu.head;
            if ins_done.contains(&head) {
                break;
            } else {
                ins_done.insert(head);
            }
        }
        dbg!(cpu.acc);
    }
    #[test]
    fn part2() {
        let input = fs::read_to_string("input/day08").unwrap();
        let cpu = CPU::new(&input);
        'outer: for i in 0..cpu.instructions.len() {
            if let Op::Acc(_) = cpu.instructions[i] {
                continue;
            }
            let mut cpu_fix = cpu.clone();
            cpu_fix.instructions[i] = match cpu_fix.instructions[i] {
                Op::Jmp(n) => Op::Nop(n),
                Op::Acc(_) => {
                    unreachable!()
                }
                Op::Nop(n) => Op::Jmp(n),
            };
            let mut ins_done = HashSet::new();
            ins_done.insert(0);
            while cpu_fix.run() {
                if ins_done.contains(&cpu_fix.head) {
                    continue 'outer;
                } else {
                    ins_done.insert(cpu_fix.head);
                }
            }
            println!("answer part2: {}", cpu_fix.acc);
        }
    }
}
