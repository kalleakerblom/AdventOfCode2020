#[derive(Clone, Copy)]
enum State {
    Floor,
    Empty,
    Occupied,
}
struct Game(Vec<Vec<State>>);

impl Game {
    fn new(input: &str) -> Self {
        let map = input
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| match c {
                        '.' => State::Floor,
                        'L' => State::Empty,
                        _ => panic!(),
                    })
                    .collect()
            })
            .collect();
        Game(map)
    }
    fn occupied_seats(&self) -> usize {
        self.0
            .iter()
            .map(|row| {
                row.iter()
                    .map(|s| if let State::Occupied = s { 1 } else { 0 })
                    .sum::<usize>()
            })
            .sum()
    }
    fn step(self) -> Self {
        let mut next_map =
            vec![vec![State::Floor; self.0[0].len()]; self.0.len()];
        self.0
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter().enumerate().map(move |(x, tile)| (x, y, tile))
            })
            .for_each(|(x, y, tile)| {
                next_map[y][x] = self.next_state(*tile, x, y)
            });

        Game(next_map)
    }
    fn next_state(&self, tile: State, x: usize, y: usize) -> State {
        match tile {
            State::Floor => State::Floor,
            State::Empty => {
                if self.occupied_neighbors(x, y) == 0 {
                    State::Occupied
                } else {
                    State::Empty
                }
            }
            State::Occupied => {
                if self.occupied_neighbors(x, y) >= 4 {
                    State::Empty
                } else {
                    State::Occupied
                }
            }
        }
    }
    fn occupied_neighbors(&self, x: usize, y: usize) -> usize {
        let x = x as i64;
        let y = y as i64;
        [
            (x - 1, y - 1),
            (x, y - 1),
            (x + 1, y - 1),
            (x - 1, y),
            (x + 1, y),
            (x - 1, y + 1),
            (x, y + 1),
            (x + 1, y + 1),
        ]
        .iter()
        .map(
            |&pos| {
                if let Some(State::Occupied) = self.get_tile(pos) {
                    1
                } else {
                    0
                }
            },
        )
        .sum()
    }
    fn get_tile(&self, (x, y): (i64, i64)) -> Option<State> {
        let max_x = (self.0[0].len() - 1) as i64;
        let max_y = (self.0.len() - 1) as i64;
        if 0 <= x && x <= max_x && 0 <= y && y <= max_y {
            Some(self.0[y as usize][x as usize])
        } else {
            None
        }
    }
    /////// PART 2 ////////////
    fn occupied_in_view(&self, pos: &(i64, i64)) -> usize {
        [(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)]
            .iter()
            .map(|dir| {
                if let Some(State::Occupied) = self.first_seat_in_dir(pos, dir)
                {
                    1
                } else {
                    0
                }
            })
            .sum()
    }
    fn first_seat_in_dir(
        &self,
        pos: &(i64, i64),
        dir: &(i64, i64),
    ) -> Option<State> {
        let add = |a: &(i64, i64), b: &(i64, i64)| (a.0 + b.0, a.1 + b.1);
        let mut view = add(pos, dir);
        while let Some(State::Floor) = self.get_tile(view) {
            view = add(&view, dir);
        }
        self.get_tile(view)
    }

    fn next_state_part2(&self, tile: State, x: usize, y: usize) -> State {
        match tile {
            State::Floor => State::Floor,
            State::Empty => {
                if self.occupied_in_view(&(x as i64, y as i64)) == 0 {
                    State::Occupied
                } else {
                    State::Empty
                }
            }
            State::Occupied => {
                if self.occupied_in_view(&(x as i64, y as i64)) >= 5 {
                    State::Empty
                } else {
                    State::Occupied
                }
            }
        }
    }
    fn step2(self) -> Self {
        let mut next_map =
            vec![vec![State::Floor; self.0[0].len()]; self.0.len()];
        self.0
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter().enumerate().map(move |(x, tile)| (x, y, tile))
            })
            .for_each(|(x, y, tile)| {
                next_map[y][x] = self.next_state_part2(*tile, x, y)
            });

        Game(next_map)
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::Game;
    #[test]
    fn example() {
        let input = fs::read_to_string("input/example11").unwrap();
        let mut g = Game::new(&input);
        for _ in 0..100 {
            g = g.step();
        }
        let ans = g.occupied_seats();
        assert_eq!(ans, 37);
    }
    #[test]
    fn part1() {
        let input = fs::read_to_string("input/day11").unwrap();
        let mut g = Game::new(&input);
        for _ in 0..100 {
            g = g.step();
        }
        let ans = g.occupied_seats();
        assert_eq!(ans, 2222);
    }
    #[test]
    fn example2() {
        let input = fs::read_to_string("input/example11").unwrap();
        let mut g = Game::new(&input);
        for _ in 0..100 {
            g = g.step2();
        }
        let ans = g.occupied_seats();
        assert_eq!(ans, 26);
    }
    #[test]
    fn part2() {
        let input = fs::read_to_string("input/day11").unwrap();
        let mut g = Game::new(&input);
        for _ in 0..100 {
            g = g.step2();
        }
        let ans = g.occupied_seats();
        assert_eq!(ans, 2032);
    }
}
