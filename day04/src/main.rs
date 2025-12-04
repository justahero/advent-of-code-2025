#![allow(dead_code)]

#[derive(Debug, Clone, Copy)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone)]
struct Floor {
    width: i32,
    height: i32,
    lines: Vec<u8>,
}

impl Floor {
    const EMPTY: u8 = b'.';
    const ROLL: u8 = b'@';

    pub fn new() -> Self {
        Self {
            width: 0,
            height: 0,
            lines: Vec::new(),
        }
    }

    /// Find all locations a forklift can move to.
    ///
    /// A forklift can only be placed on a tile with a roll on it.
    pub fn find_locations(&self) -> Vec<Pos> {
        let mut forklifts = Vec::new();
        for y in 0..self.height() {
            for x in 0..self.width() {
                let tile = self.get(x, y);
                if let Some(Self::ROLL) = tile {
                    let count = self
                        .neighbors(x, y)
                        .iter()
                        .filter(|(_, tile)| *tile == Self::ROLL)
                        .count();
                    if count < 4 {
                        forklifts.push(Pos::new(x, y));
                    }
                }
            }
        }
        forklifts
    }

    /// Remove all forklift location from the new Floor.
    pub fn build_updated_floor(&self, forklifts: Vec<Pos>) -> Floor {
        let mut floor = self.clone();

        for Pos { x, y } in forklifts {
            if let Some(tile) = floor.get_mut(x, y) {
                *tile = Self::EMPTY;
            }
        }

        floor
    }

    pub fn add_row(&mut self, mut line: Vec<u8>) {
        debug_assert!(self.width == 0 || line.len() as i32 == self.width);
        self.width = line.len() as i32;
        self.height += 1;
        self.lines.append(&mut line);
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    const NEIGHBORS: [(i32, i32); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    /// Returns the set of neighbors with their position & tile value.
    pub fn neighbors(&self, x: i32, y: i32) -> Vec<(Pos, u8)> {
        let mut result = Vec::new();
        for (i, j) in Self::NEIGHBORS.iter() {
            let neighbor = Pos::new(x + i, y + j);
            if let Some(tile) = self.get(neighbor.x, neighbor.y) {
                result.push((neighbor, tile));
            }
        }
        result
    }

    fn get(&self, x: i32, y: i32) -> Option<u8> {
        if 0 <= x && x < self.width as i32 && 0 <= y && y < self.height as i32 {
            Some(self.lines[(y * self.width + x) as usize])
        } else {
            None
        }
    }

    fn get_mut(&mut self, x: i32, y: i32) -> Option<&mut u8> {
        if 0 <= x && x < self.width as i32 && 0 <= y && y < self.height as i32 {
            self.lines.get_mut((y * self.width + x) as usize)
        } else {
            None
        }
    }
}

fn parse_input(input: &str) -> Floor {
    let floor = input
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(|line| line.as_bytes())
        .fold(Floor::new(), |mut floor, row| {
            floor.add_row(row.to_vec());
            floor
        });

    floor
}

fn process_part1(floor: Floor) -> usize {
    floor.find_locations().len()
}

fn process_part2(mut floor: Floor) -> usize {
    let mut total = 0;

    loop {
        let forklifts = floor.find_locations();
        if forklifts.is_empty() {
            break;
        }

        total += forklifts.len();
        floor = floor.build_updated_floor(forklifts);
    }

    total
}

fn main() {
    let floor = parse_input(include_str!("input.txt"));
    let result = process_part1(floor.clone());
    println!("PART 1: {}", result);
    let result = process_part2(floor);
    println!("PART 2: {}", result);
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, process_part1, process_part2};

    const INPUT: &str = r#"
        ..@@.@@@@.
        @@@.@.@.@@
        @@@@@.@.@@
        @.@@@@..@.
        @@.@@@@.@@
        .@@@@@@@.@
        .@.@.@.@@@
        @.@@@.@@@@
        .@@@@@@@@.
        @.@.@@@.@.
    "#;

    #[test]
    fn test_parse_floor() {
        let floor = parse_input(INPUT);
        assert_eq!(10, floor.width());
        assert_eq!(10, floor.height());
    }

    #[test]
    fn check_part1() {
        let floor = parse_input(INPUT);
        assert_eq!(13, process_part1(floor));
    }

    #[test]
    fn check_part2() {
        let floor = parse_input(INPUT);
        assert_eq!(43, process_part2(floor));
    }
}
