use cgmath::Vector3;
use cgmath::Vector4;
use itertools::Itertools;
use std::collections::HashSet;

type V3d = Vector3<i64>;
type V4d = Vector4<i64>;
#[derive(Debug)]
struct ConwayCube(HashSet<V3d>);

impl ConwayCube {
    fn parse(input: &str) -> Self {
        let map = input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().filter_map(move |(x, c)| {
                    if c == '#' {
                        Some(V3d::new(x as i64, y as i64, 0))
                    } else {
                        None
                    }
                })
            })
            .collect();
        Self(map)
    }
    fn run(&mut self) {
        let mut next_map = HashSet::with_capacity(self.0.len());
        let mut to_visit = HashSet::with_capacity(self.0.len());
        for pos in self.0.iter() {
            let neighbors = get_neighbors_3d(pos);
            to_visit.extend(neighbors.iter().cloned());
        }
        for visit in to_visit.iter() {
            let neighbors = get_neighbors_3d(visit);
            let alive_neighbors =
                neighbors.iter().filter(|n| self.0.contains(n)).count();
            if (self.0.contains(visit) && (2..=3).contains(&alive_neighbors))
                || (!self.0.contains(visit) && alive_neighbors == 3)
            {
                next_map.insert(*visit);
            }
        }
        self.0 = next_map;
    }
}

fn get_neighbors_3d(p: &V3d) -> [V3d; 26] {
    let mut res = [V3d::new(0, 0, 0); 26];
    let x_vals = [p.x, p.x - 1, p.x + 1];
    let y_vals = [p.y, p.y - 1, p.y + 1];
    let z_vals = [p.z, p.z - 1, p.z + 1];
    x_vals
        .iter()
        .cartesian_product(y_vals.iter().cartesian_product(z_vals.iter()))
        .skip(1)
        .enumerate()
        .for_each(|(i, (x, (y, z)))| res[i] = V3d::new(*x, *y, *z));
    res
}
#[derive(Debug)]
struct ConwayHyperCube(HashSet<V4d>);

impl ConwayHyperCube {
    fn parse(input: &str) -> Self {
        let map = input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().filter_map(move |(x, c)| {
                    if c == '#' {
                        Some(V4d::new(x as i64, y as i64, 0, 0))
                    } else {
                        None
                    }
                })
            })
            .collect();
        Self(map)
    }
    fn run(&mut self) {
        let mut next_map = HashSet::with_capacity(self.0.len());
        let mut to_visit = HashSet::with_capacity(self.0.len());
        for pos in self.0.iter() {
            let neighbors = get_neighbors_4d(pos);
            to_visit.extend(neighbors.iter().cloned());
        }
        for visit in to_visit.iter() {
            let neighbors = get_neighbors_4d(visit);
            let alive_neighbors =
                neighbors.iter().filter(|n| self.0.contains(n)).count();
            if (self.0.contains(visit) && (2..=3).contains(&alive_neighbors))
                || (!self.0.contains(visit) && alive_neighbors == 3)
            {
                next_map.insert(*visit);
            }
        }
        self.0 = next_map;
    }
}

fn get_neighbors_4d(p: &V4d) -> [V4d; 80] {
    let mut res = [V4d::new(0, 0, 0, 0); 80];
    let x_vals = [p.x, p.x - 1, p.x + 1];
    let y_vals = [p.y, p.y - 1, p.y + 1];
    let z_vals = [p.z, p.z - 1, p.z + 1];
    let w_vals = [p.w, p.w - 1, p.w + 1];
    x_vals
        .iter()
        .cartesian_product(
            y_vals.iter().cartesian_product(
                z_vals.iter().cartesian_product(w_vals.iter()),
            ),
        )
        .skip(1)
        .enumerate()
        .for_each(|(i, (x, (y, (z, w))))| res[i] = V4d::new(*x, *y, *z, *w));
    res
}
#[cfg(test)]
mod tests {
    use std::fs;

    use super::{ConwayCube, ConwayHyperCube};
    #[test]
    fn example() {
        let input = fs::read_to_string("input/example17").unwrap();
        let mut map = ConwayCube::parse(&input);
        for _ in 0..6 {
            map.run()
        }
        assert_eq!(map.0.len(), 112);
    }
    #[test]
    fn part1() {
        let input = fs::read_to_string("input/day17").unwrap();
        let mut map = ConwayCube::parse(&input);
        for _ in 0..6 {
            map.run()
        }
        assert_eq!(map.0.len(), 247);
    }
    #[test]
    fn example_part2() {
        let input = fs::read_to_string("input/example17").unwrap();
        let mut map = ConwayHyperCube::parse(&input);
        for _ in 0..6 {
            map.run()
        }
        assert_eq!(map.0.len(), 848);
    }
    #[test]
    fn part2() {
        let input = fs::read_to_string("input/day17").unwrap();
        let mut map = ConwayHyperCube::parse(&input);
        for _ in 0..6 {
            map.run()
        }
        assert_eq!(map.0.len(), 1392);
    }
}
