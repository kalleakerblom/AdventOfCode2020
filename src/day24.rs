use std::collections::{HashMap, HashSet};
type Pos = cgmath::Vector2<i32>;

enum Dir {
    E,
    W,
    NW,
    NE,
    SW,
    SE,
}

#[derive(PartialEq)]
enum Tile {
    Black,
    White,
}

fn move_dir(pos: Pos, dir: Dir) -> Pos {
    match dir {
        Dir::E => pos + Pos::new(2, 0),
        Dir::W => pos + Pos::new(-2, 0),
        Dir::NW => pos + Pos::new(-1, 1),
        Dir::NE => pos + Pos::new(1, 1),
        Dir::SW => pos + Pos::new(-1, -1),
        Dir::SE => pos + Pos::new(1, -1),
    }
}

fn get_tile_pos(instruction: &str) -> Pos {
    let mut pos = Pos::new(0, 0);
    let mut char_iter = instruction.chars().peekable();
    loop {
        if char_iter.peek().filter(|c| "ns".contains(**c)).is_some() {
            match (char_iter.next(), char_iter.next()) {
                (Some('n'), Some('w')) => pos = move_dir(pos, Dir::NW),
                (Some('n'), Some('e')) => pos = move_dir(pos, Dir::NE),
                (Some('s'), Some('w')) => pos = move_dir(pos, Dir::SW),
                (Some('s'), Some('e')) => pos = move_dir(pos, Dir::SE),
                _ => panic!("Unexpected move symbol."),
            }
        } else {
            match char_iter.next() {
                Some('w') => pos = move_dir(pos, Dir::W),
                Some('e') => pos = move_dir(pos, Dir::E),
                Some(_) => panic!("Unexpected move symbol."),
                None => break,
            }
        }
    }
    pos
}

fn flip_tiles(instructions: &str) -> HashMap<Pos, Tile> {
    let mut flipped_map: HashMap<Pos, Tile> = HashMap::new();
    for instruction in instructions.lines() {
        let pos = get_tile_pos(instruction);
        let entry = flipped_map.entry(pos).or_insert(Tile::White);
        *entry = match entry {
            Tile::Black => Tile::White,
            Tile::White => Tile::Black,
        };
    }
    flipped_map
}
// Part 2
fn get_neighboring_tiles(pos: Pos) -> [Pos; 6] {
    [
        move_dir(pos, Dir::E),
        move_dir(pos, Dir::W),
        move_dir(pos, Dir::NE),
        move_dir(pos, Dir::SE),
        move_dir(pos, Dir::NW),
        move_dir(pos, Dir::SW),
    ]
}

fn flip_tiles_part2(tiles: &mut HashMap<cgmath::Vector2<i32>, Tile>) {
    let to_check: HashSet<Pos> = tiles
        .iter()
        .filter(|&(_, tile)| *tile == Tile::Black)
        .flat_map(|(&pos, _)| std::array::IntoIter::new(get_neighboring_tiles(pos)))
        .collect();
    let new_tiles = to_check
        .iter()
        .map(|&pos| {
            let count = get_neighboring_tiles(pos)
                .iter()
                .filter(|near| matches!(tiles.get(near), Some(Tile::Black)))
                .count();
            match (tiles.entry(pos).or_insert(Tile::White), count) {
                (Tile::Black, 1) | (Tile::Black, 2) | (Tile::White, 2) => (pos, Tile::Black),
                (Tile::Black, _) | (Tile::White, _) => (pos, Tile::White),
            }
        })
        .collect();
    *tiles = new_tiles;
}

fn flip_game(mut tiles: HashMap<Pos, Tile>, turns: usize) -> HashMap<Pos, Tile> {
    for _ in 0..turns {
        flip_tiles_part2(&mut tiles);
    }
    tiles
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::day24::{flip_game, flip_tiles, Tile};
    #[test]
    fn example() {
        let inp = fs::read_to_string("input/example24").unwrap();
        let map = flip_tiles(&inp);
        assert_eq!(map.iter().filter(|(_, v)| **v == Tile::Black).count(), 10);
    }

    #[test]
    fn part1() {
        let inp = fs::read_to_string("input/day24").unwrap();
        let map = flip_tiles(&inp);
        assert_eq!(map.iter().filter(|(_, v)| **v == Tile::Black).count(), 434);
    }

    #[test]
    fn example2() {
        let inp = fs::read_to_string("input/example24").unwrap();
        let map = crate::day24::flip_tiles(&inp);
        let game_map = flip_game(map, 100);
        assert_eq!(
            game_map.iter().filter(|(_, v)| **v == Tile::Black).count(),
            2208
        );
    }
    #[test]
    fn part2() {
        let inp = fs::read_to_string("input/day24").unwrap();
        let map = crate::day24::flip_tiles(&inp);
        let game_map = flip_game(map, 100);
        assert_eq!(
            game_map.iter().filter(|(_, v)| **v == Tile::Black).count(),
            3955
        );
    }
}
