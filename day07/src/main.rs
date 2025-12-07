#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use std::fmt::{Display, Formatter, Result};

const EMPTY: u8 = b'.';
const START: u8 = b'S';
const SPLITTER: u8 = b'^';
const BEAM: u8 = b'|';

#[derive(Debug, Clone, Copy, PartialEq)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Clone)]
struct Grid {
    width: u32,
    height: u32,
    fields: Vec<u8>,
    start: Option<Pos>,
}

impl Grid {
    pub fn new() -> Self {
        Self {
            width: 0,
            height: 0,
            fields: Vec::new(),
            start: None,
        }
    }

    pub fn set_beam(&mut self, pos: Pos) {
        if let Some(field) = self.get_mut(pos) {
            *field = BEAM;
        }
    }

    pub fn set_start(&mut self, pos: Pos) {
        self.start = Some(pos);
    }

    pub fn add_row(&mut self, line: &[u8]) {
        debug_assert!(self.width == 0 || self.width == line.len() as u32);
        self.width = line.len() as u32;
        self.height += 1;

        let mut line = line.to_vec();
        self.fields.append(&mut line);
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn start(&self) -> Option<&Pos> {
        self.start.as_ref()
    }

    fn get(&self, Pos { x, y }: Pos) -> Option<u8> {
        if 0 <= x && x < self.width as i32 && 0 <= y && y < self.height as i32 {
            Some(self.fields[(y * self.width as i32 + x) as usize])
        } else {
            None
        }
    }

    fn get_mut(&mut self, Pos { x, y }: Pos) -> Option<&mut u8> {
        if 0 <= x && x < self.width as i32 && 0 <= y && y < self.height as i32 {
            self.fields.get_mut((y * self.width as i32 + x) as usize)
        } else {
            None
        }
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let value = char::from(self.fields[(y * self.width + x) as usize]);
                write!(f, "{}", value)?;
            }
            writeln!(f, "")?;
        }
        writeln!(f, "")
    }
}

fn parse_input(input: &str) -> Grid {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .enumerate()
        .fold(Grid::new(), |mut grid, (row, line)| {
            if let Some(index) = line.find('S') {
                grid.set_start(Pos::new(index as i32, row as i32));
            }
            grid.add_row(line.as_bytes());
            grid
        })
}

fn process_part1(mut grid: Grid) -> u64 {
    let mut total_splits = 0;

    let start = grid.start.expect("Failed to find start");
    let mut queue = vec![start];

    while let Some(beam) = queue.pop() {
        let pos = Pos::new(beam.x, beam.y + 1);
        if let Some(bottom) = grid.get(pos) {
            match bottom {
                b'|' => (),
                b'^' => {
                    let left = Pos::new(beam.x - 1, beam.y + 1);
                    let right = Pos::new(beam.x + 1, beam.y + 1);
                    queue.push(left);
                    queue.push(right);
                    grid.set_beam(left);
                    grid.set_beam(right);
                    total_splits += 1;
                }
                b'.' => {
                    queue.push(pos);
                    grid.set_beam(pos);
                }
                _ => (),
            }
        }

        // println!("GRID:\n{}", grid);
    }

    // TODO: apply graph algorithm
    total_splits
}

fn main() {
    let grid = parse_input(include_str!("input.txt"));
    let result = process_part1(grid);
}

#[cfg(test)]
mod tests {
    use crate::{Pos, parse_input, process_part1};

    const INPUT: &str = r#"
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
    "#;

    #[test]
    fn test_parse() {
        let grid = parse_input(INPUT);
        assert_eq!(15, grid.width());
        assert_eq!(16, grid.height());
    }

    #[test]
    fn test_process_part1() {
        let grid = parse_input(INPUT);
        assert_eq!(Some(&Pos::new(7, 0)), grid.start());
        assert_eq!(21, process_part1(grid));
    }
}
