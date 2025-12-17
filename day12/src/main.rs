#![allow(dead_code)]

use std::{collections::HashSet, fmt::Display, ops::Index, str::FromStr};

use nom::{IResult, Parser, bytes::complete::tag, multi::separated_list1};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Shape {
    /// 3x3 grid
    grid: [u8; 9],
}

impl Shape {
    /// The list of all variants a 3x3 block can be rotated / flipped to.
    const VARIANTS: [[u8; 9]; 8] = [
        [0, 1, 2, 3, 4, 5, 6, 7, 8],
        [6, 3, 0, 7, 4, 1, 8, 5, 2],
        [8, 7, 6, 5, 4, 3, 2, 1, 0],
        [2, 5, 8, 1, 4, 7, 0, 3, 6],
        [6, 7, 8, 3, 4, 5, 0, 1, 2],
        [8, 5, 2, 7, 4, 1, 6, 3, 0],
        [2, 1, 0, 5, 4, 3, 8, 7, 6],
        [0, 3, 6, 1, 4, 7, 2, 5, 8],
    ];

    pub fn new(bytes: &[u8]) -> Self {
        debug_assert!(bytes.len() == 9);
        Self {
            grid: bytes.try_into().unwrap(),
        }
    }

    /// Returns a list of all shape variants
    pub fn variants(&self) -> HashSet<Shape> {
        HashSet::from_iter(Self::VARIANTS.iter().map(|variant| {
            Shape::from_iter(variant.iter().map(|index| self.grid[*index as usize]))
        }))
    }
}

impl Index<usize> for Shape {
    type Output = [u8];

    fn index(&self, y: usize) -> &Self::Output {
        &self.grid[y * 3..y * 3 + 3]
    }
}

impl Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..3 {
            for x in 0..3 {
                let index = y * 3 + x;
                write!(f, "{}", self.grid[index])?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

impl FromIterator<u8> for Shape {
    fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
        let bytes: Vec<u8> = iter.into_iter().collect();
        Self::new(bytes.as_slice())
    }
}

impl FromStr for Shape {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let grid = input
            .lines()
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
    regions: Vec<(Region, Vec<u8>)>,
}

fn parse_u16(input: &str) -> IResult<&str, u16> {
    nom::character::complete::u16(input)
}

fn parse_u8(input: &str) -> IResult<&str, u8> {
    nom::character::complete::u8(input)
}

fn parse_region(line: &str) -> (Region, Vec<u8>) {
    let (_, (width, _, height, _, indices)) = (
        parse_u16,
        tag("x"),
        parse_u16,
        tag(": "),
        separated_list1(tag(" "), parse_u8),
    )
        .parse(line)
        .expect("Failed to parse line");
    (Region { width, height }, indices)
}

fn parse_input(input: &str) -> TreeFarm {
    let lines = input.split("\n\n").collect::<Vec<_>>();
    let (trees, shapes) = lines.split_last().expect("failed to get last block");

    let shapes = shapes
        .iter()
        .map(|block| {
            let (_index, block) = block.split_once("\n").unwrap();
            block.parse::<Shape>().expect("Failed to parse shape")
        })
        .collect::<Vec<_>>();

    let regions: Vec<(Region, Vec<u8>)> = trees.lines().map(parse_region).collect();
    TreeFarm { shapes, regions }
}

fn main() {
    // TODO
}

#[cfg(test)]
mod tests {
    use crate::{Shape, parse_input, parse_region};

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
    fn test_parse_region() {
        let input = "12x5: 1 0 1 0 2 2";
        let (region, indices) = parse_region(input);
        assert_eq!(vec![1, 0, 1, 0, 2, 2], indices);
        assert_eq!(12, region.width);
        assert_eq!(5, region.height);
    }

    #[test]
    fn test_parse_shape() {
        let input: &str = r#"###
#..
###"#;
        assert!(input.parse::<Shape>().is_ok());
    }

    #[test]
    fn test_shape_variants() {
        let input: &str = "#..\n#.#\n...";
        assert_eq!(8, input.parse::<Shape>().unwrap().variants().len());
        let input: &str = "###\n###\n.#.";
        assert_eq!(4, input.parse::<Shape>().unwrap().variants().len());
        let input: &str = "###\n###\n.#.";
        assert_eq!(4, input.parse::<Shape>().unwrap().variants().len());
        let input: &str = "#.#\n###\n#.#";
        assert_eq!(2, input.parse::<Shape>().unwrap().variants().len());
    }

    #[test]
    fn test_part1() {
        // TODO
    }
}
