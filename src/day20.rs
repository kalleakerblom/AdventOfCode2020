use scan_fmt::scan_fmt;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct SideId(u64);

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct TileId(u64);

type SideMap = HashMap<SideId, Vec<TileId>>;

#[derive(Copy, Clone)]
enum Orientation {
    Twelve,
    Three,
    Six,
    Nine,
}

#[derive(Copy, Clone)]
enum Flipped {
    True,
    False,
}

struct Tile {
    id: TileId,
    map: Vec<Vec<bool>>,
    side_ids: [(SideId, Flipped); 4], //< bool is true if side is flipped
}

impl Tile {
    fn parse(input: &str) -> Self {
        let mut lines = input.lines();
        let id: u64 = scan_fmt!(lines.next().unwrap(), "Tile {}:", u64).unwrap();
        let id = TileId(id);
        let mut map = Vec::new();
        for line in lines {
            let row: Vec<_> = line
                .chars()
                .map(|c| match c {
                    '#' => true,
                    '.' => false,
                    _ => panic!(),
                })
                .collect();
            map.push(row);
        }
        let top = map.first().unwrap();
        let bottom = map.last().unwrap();
        let left: Vec<_> = map.iter().map(|row| row[0]).collect();
        let right: Vec<_> = map.iter().map(|row| *row.last().unwrap()).collect();
        let side_ids = [
            calculate_side_id(top),
            calculate_side_id(&right),
            calculate_side_id(bottom),
            calculate_side_id(&left),
        ];
        Self { id, map, side_ids }
    }
    fn get_opposite_side_id(&self, s_id: &SideId) -> (SideId, Flipped) {
        let pos = self
            .side_ids
            .iter()
            .position(|(self_s_id, _)| self_s_id == s_id)
            .expect("invalid side id");
        self.side_ids[(pos + 2) % 4]
    }
}

fn calculate_side_id(side: &[bool]) -> (SideId, Flipped) {
    let s_id = side
        .iter()
        .fold(0, |acc, b| if *b { (acc << 1) + 1 } else { acc << 1 });

    let rev_id = side
        .iter()
        .rev()
        .fold(0, |acc, b| if *b { (acc << 1) + 1 } else { acc << 1 });
    if rev_id < s_id {
        (SideId(rev_id), Flipped::True)
    } else {
        (SideId(rev_id), Flipped::False)
    }
}

fn map_sides(tiles: &mut dyn Iterator<Item = &Tile>) -> SideMap {
    let mut side_map = SideMap::new();
    for tile in tiles {
        for (si, _) in tile.side_ids {
            side_map.entry(si).or_default().push(tile.id);
        }
    }
    side_map
}

fn find_corner_tiles(side_map: &SideMap) -> Vec<TileId> {
    let mut open_side_counts = HashMap::<TileId, u8>::new();
    for tiles in side_map.values() {
        if tiles.len() == 1 {
            *open_side_counts.entry(tiles[0]).or_default() += 1;
        }
    }
    open_side_counts
        .iter()
        .filter(|(_id, count)| **count == 2)
        .map(|(id, _)| id)
        .cloned()
        .collect()
}

#[derive(Copy, Clone)]
struct TileDescription {
    id: TileId,
    rot: Orientation,
    flip: Flipped,
}

fn find_next_right(
    current: &TileDescription,
    tiles: &HashMap<TileId, Tile>,
    side_map: &SideMap,
) -> Option<TileDescription> {
    todo!()
}

fn find_next_down(
    current: &TileDescription,
    tiles: &HashMap<TileId, Tile>,
    side_map: &SideMap,
) -> Option<TileDescription> {
    todo!()
}

#[allow(unused_variables, unused_mut)]
fn assemble_map(
    tiles: &HashMap<TileId, Tile>,
    side_map: &SideMap,
    start_tile: TileId,
) -> Vec<Vec<TileDescription>> {
    //start in one of the corners, build row by row.
    let mut result = Vec::new();
    let mut next_down = Some(TileDescription {
        id: start_tile,
        rot: Orientation::Twelve,
        flip: Flipped::False,
    });
    while let Some(first) = next_down {
        let mut row = vec![first];
        let mut next_right = find_next_right(&first, tiles, side_map);
        while let Some(next) = next_right {
            row.push(next);
            next_right = find_next_right(&next, tiles, side_map);
        }
        result.push(row);
        next_down = find_next_down(&first, tiles, side_map);
    }

    result
}
#[cfg(test)]
mod tests {
    use crate::day20::*;
    use std::fs;
    #[test]
    fn example() {
        let input = fs::read_to_string("input/example20").unwrap();
        let tiles: Vec<_> = input.split("\r\n\r\n").map(|s| Tile::parse(s)).collect();
        let map = map_sides(&mut tiles.iter());
        let ans = find_corner_tiles(&map);
        assert_eq!(ans.iter().fold(1, |acc, id| acc * id.0), 20899048083289);
    }
    #[test]
    fn part1() {
        let input = fs::read_to_string("input/day20").unwrap();
        let tiles: Vec<_> = input.split("\r\n\r\n").map(|s| Tile::parse(s)).collect();
        let map = map_sides(&mut tiles.iter());
        let ans = find_corner_tiles(&map);
        assert_eq!(ans.iter().fold(1, |acc, id| acc * id.0), 23386616781851);
    }
    #[test]
    fn part2() {
        let input = fs::read_to_string("input/day20").unwrap();
        let tile_map: HashMap<_, _> = input
            .split("\r\n\r\n")
            .map(|s| Tile::parse(s))
            .map(|t| (t.id, t))
            .collect();
        let side_map = map_sides(&mut tile_map.values());
        let ans = find_corner_tiles(&side_map);
        let start_tile = ans[0];
        //TODO: Implement this
        let _big_map = assemble_map(&tile_map, &side_map, start_tile);
        //TODO: turn into big map of bools and search for loch-ness
        // TODO: Rotate/flip the monsta if no monsta was found.
    }
}
