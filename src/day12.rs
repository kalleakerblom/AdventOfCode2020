use cgmath::Vector2;
type V2d = Vector2<i64>;

enum Inst {
    N(i64),
    S(i64),
    E(i64),
    W(i64),
    L(i64),
    R(i64),
    F(i64),
}

fn parse_instructions(input: &str) -> Vec<Inst> {
    input
        .lines()
        .map(|l| {
            let (c, n) = l.split_at(1);
            match c {
                "N" => Inst::N(n.parse().unwrap()),
                "S" => Inst::S(n.parse().unwrap()),
                "E" => Inst::E(n.parse().unwrap()),
                "W" => Inst::W(n.parse().unwrap()),
                "L" => Inst::L(n.parse().unwrap()),
                "R" => Inst::R(n.parse().unwrap()),
                "F" => Inst::F(n.parse().unwrap()),
                _ => panic!("unkown symbol"),
            }
        })
        .collect()
}

struct Ship {
    pos: V2d,
    dir: V2d,
}

impl Ship {
    fn new(pos: V2d, dir: V2d) -> Self {
        Self { pos, dir }
    }
    fn turn_left(&mut self) {
        self.dir = V2d::new(-self.dir.y, self.dir.x);
    }
    fn turn_right(&mut self) {
        self.dir = V2d::new(self.dir.y, -self.dir.x);
    }
    fn run(&mut self, instructions: &[Inst]) {
        for inst in instructions {
            match inst {
                Inst::N(n) => self.pos += *n * V2d::new(0, 1),
                Inst::S(n) => self.pos += *n * V2d::new(0, -1),
                Inst::E(n) => self.pos += *n * V2d::new(1, 0),
                Inst::W(n) => self.pos += *n * V2d::new(-1, 0),
                Inst::L(n) => {
                    let turns = n / 90;
                    for _ in 0..turns {
                        self.turn_left();
                    }
                }
                Inst::R(n) => {
                    let turns = n / 90;
                    for _ in 0..turns {
                        self.turn_right();
                    }
                }
                Inst::F(n) => {
                    self.pos += *n * self.dir;
                }
            }
        }
    }
    fn run_part2(&mut self, instructions: &[Inst]) {
        for inst in instructions {
            match inst {
                Inst::N(n) => self.dir += *n * V2d::new(0, 1),
                Inst::S(n) => self.dir += *n * V2d::new(0, -1),
                Inst::E(n) => self.dir += *n * V2d::new(1, 0),
                Inst::W(n) => self.dir += *n * V2d::new(-1, 0),
                Inst::L(n) => {
                    let turns = n / 90;
                    for _ in 0..turns {
                        self.turn_left();
                    }
                }
                Inst::R(n) => {
                    let turns = n / 90;
                    for _ in 0..turns {
                        self.turn_right();
                    }
                }
                Inst::F(n) => {
                    self.pos += *n * self.dir;
                }
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use std::fs;

    use fs::read_to_string;

    use super::{parse_instructions, Ship, V2d};
    #[test]
    fn example() {
        let input = "F10\nN3\nF7\nR90\nF11".to_string();
        let instructions = parse_instructions(&input);
        let mut ship = Ship::new(V2d::new(0, 0), V2d::new(1, 0));
        ship.run(&instructions);
        let ans = ship.pos.x.abs() + ship.pos.y.abs();
        assert_eq!(ans, 25);
    }
    #[test]
    fn part1() {
        let input = read_to_string("input/day12").unwrap();
        let instructions = parse_instructions(&input);
        let mut ship = Ship::new(V2d::new(0, 0), V2d::new(1, 0));
        ship.run(&instructions);
        let ans = ship.pos.x.abs() + ship.pos.y.abs();
        assert_eq!(ans, 1148);
    }
    #[test]
    fn example2() {
        let input = "F10\nN3\nF7\nR90\nF11".to_string();
        let instructions = parse_instructions(&input);
        let mut ship = Ship::new(V2d::new(0, 0), V2d::new(10, 1));
        ship.run_part2(&instructions);
        let ans = ship.pos.x.abs() + ship.pos.y.abs();
        assert_eq!(ans, 286);
    }
    #[test]
    fn part2() {
        let input = read_to_string("input/day12").unwrap();
        let instructions = parse_instructions(&input);
        let mut ship = Ship::new(V2d::new(0, 0), V2d::new(10, 1));
        ship.run_part2(&instructions);
        let ans = ship.pos.x.abs() + ship.pos.y.abs();
        assert_eq!(ans, 52203);
    }
}
