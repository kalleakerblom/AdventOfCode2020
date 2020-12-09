use std::collections::HashMap;

enum Tile {
    Tree,
    Ground,
}
struct Map {
    tiles: HashMap<(usize, usize), Tile>,
    height: usize,
    width: usize,
}

impl Map {
    fn trees_hit(&self, (dir_x, dir_y): (usize, usize)) -> u32 {
        let mut hit = 0;
        let mut x = dir_x;
        let mut y = dir_y;
        while y < self.height {
            let pos = (x % self.width, y);
            match self.tiles[&pos] {
                Tile::Tree => hit += 1,
                Tile::Ground => {}
            }
            x += dir_x;
            y += dir_y;
        }

        hit
    }
}

fn parse(input: &str) -> Map {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().chars().count();
    let mut tiles = HashMap::with_capacity(height * width);
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            match c {
                '.' => tiles.insert((j, i), Tile::Ground),
                '#' => tiles.insert((j, i), Tile::Tree),
                _ => panic!("unknown symbol"),
            };
        }
    }
    Map { tiles, height, width }
}
#[cfg(test)]
mod tests {
    use std::fs;

    use super::parse;
    #[test]
    fn example() {
        let inp = fs::read_to_string("input/example03").unwrap();
        let map = parse(&inp);
        let ans = map.trees_hit((3, 1));
        assert_eq!(ans, 7);
    }
    #[test]
    fn part1() {
        let inp = fs::read_to_string("input/day03").unwrap();
        let map = parse(&inp);
        let ans = map.trees_hit((3, 1));
        assert_eq!(ans, 280);
    }
    #[test]
    fn part2() {
        let inp = fs::read_to_string("input/day03").unwrap();
        let map = parse(&inp);
        let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
        let ans: usize =
            slopes.iter().map(|dir| map.trees_hit(*dir) as usize).product();
        assert_eq!(ans, 4355551200);
    }
}
