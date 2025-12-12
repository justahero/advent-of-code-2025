#![allow(dead_code)]

use std::{collections::HashSet, str::FromStr};

#[derive(Debug, Clone)]
struct Shape {
    /// 3x3 grid
    grid: [u8; 9],
}

impl Shape {
    pub fn new(bytes: &[u8]) -> Self {
        debug_assert!(bytes.len() == 9);
        Self {
            grid: bytes.try_into().unwrap(),
        }
    }

    /// Returns a list of all shape variants
    pub fn variants(&self) -> HashSet<Shape> {
        todo!("")
    }

    pub fn flip_x(&self) -> Self {
        todo!("")
    }
}

impl FromStr for Shape {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let grid = input
            .lines()
            .skip(1)
            .flat_map(|line| line.as_bytes())
            .cloned()
            .collect::<Vec<_>>();

        Ok(Shape::new(&grid))
    }
}

#[derive(Debug)]
struct Region {
    width: u16,
    height: u16,
}

#[derive(Debug)]
struct TreeFarm {
    shapes: Vec<Shape>,
    regions: Vec<(Region, Vec<u16>)>,
}

fn parse_input(input: &str) -> TreeFarm {
    let lines = input.split("\n\n").collect::<Vec<_>>();
    let (trees, shapes) = lines.split_last().expect("failed to get last block");

    let shapes = shapes
        .iter()
        .map(|block| block.parse::<Shape>().expect("Failed to parse shape"))
        .collect::<Vec<_>>();

    dbg!(&shapes, &trees);

    todo!("")
}

fn main() {
    // TODO
}

#[cfg(test)]
mod tests {
    use crate::{Shape, parse_input};

    const INPUT: &str = r#"0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2"#;

    #[test]
    fn test_parse() {
        let farm = parse_input(INPUT);
        assert_eq!(6, farm.shapes.len());
    }

    #[test]
    fn test_parse_shape() {
        let input: &str = r#"###
#..
###"#;
        assert!(input.parse::<Shape>().is_ok());
    }

    #[test]
    fn test_part1() {
        // TODO
    }
}
