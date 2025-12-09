#![allow(dead_code)]

use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

use itertools::Itertools;

type Pair = (usize, usize);

#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct Vec3 {
    x: i64,
    y: i64,
    z: i64,
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:3},{:3},{:3})", self.x, self.y, self.z)
    }
}

impl std::ops::Sub for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Vec3 {
    /// Returns the length of the Vec3.
    fn length(&self) -> f64 {
        let Self { x, y, z } = self;
        ((x * x + y * y + z * z) as f64).sqrt()
    }

    /// Calculates the distance between two Vec3
    fn distance(&self, rhs: &Vec3) -> f64 {
        (rhs - self).length()
    }

    /// Returns the squared distance
    fn distance_squared(&self, rhs: &Vec3) -> u64 {
        let Self { x, y, z } = rhs - self;
        (x * x + y * y + z * z) as u64
    }
}

impl FromStr for Vec3 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(',').collect::<Vec<_>>();
        if parts.len() != 3 {
            return Err("Failed to parse 3 elements".to_string());
        }

        let numbers = parts
            .iter()
            .map(|part| part.parse::<i64>().expect("Not a number"))
            .collect::<Vec<_>>();
        Ok(Self {
            x: numbers[0],
            y: numbers[1],
            z: numbers[2],
        })
    }
}

fn parse(input: &str) -> Vec<Vec3> {
    input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| line.parse::<Vec3>().expect("Failed to parse junction"))
        .collect::<Vec<_>>()
}

/// Connect junctions by shortest distance, group them into circuits.
///
/// Keep list of all circuits, stop when the given number of connections have been reached.
fn process_part1(junctions: &[Vec3], num_pairs: usize, largest: usize) -> u64 {
    println!("PROCESS max-pairs: {}", num_pairs);

    // Generate all pairings, sorted by distance between positions
    let pairings = junctions
        .iter()
        .tuple_combinations()
        .map(|(l, r)| (l.distance_squared(r), (l, r)))
        .sorted_by(|l, r| l.0.partial_cmp(&r.0).unwrap())
        .collect::<Vec<_>>();

    // Collect all circuits.
    let mut circuits: Vec<Vec<Vec3>> = Vec::new();
    for (_, (lhs, rhs)) in pairings.into_iter().take(num_pairs) {
        let left = circuits.iter().position(|c| c.contains(lhs));
        let right = circuits.iter().position(|c| c.contains(rhs));

        match (left, right) {
            (Some(l), None) => {
                circuits[l].push(rhs.clone());
            }
            (None, Some(r)) => {
                circuits[r].push(lhs.clone());
            }
            (Some(l), Some(r)) => {
                if l != r {
                    let other = circuits[r].clone();
                    circuits[l].extend(other);
                    circuits.remove(r);
                }
            }
            (None, None) => {
                circuits.push(vec![lhs.clone(), rhs.clone()]);
            }
        }
    }

    circuits
        .into_iter()
        .sorted_by(|l, r| r.len().cmp(&l.len()))
        .take(largest)
        .map(|circuit| circuit.len() as u64)
        .product()
}

fn process_part2(junctions: &[Vec3]) -> u64 {
    let pairings = junctions
        .iter()
        .tuple_combinations()
        .map(|(l, r)| (l.distance_squared(r), (l, r)))
        .sorted_by(|l, r| l.0.partial_cmp(&r.0).unwrap())
        .collect::<Vec<_>>();

    let mut last_merged_pair: Option<(Vec3, Vec3)> = None;
    let mut circuits: Vec<Vec<Vec3>> = junctions.iter().map(|j| vec![j.clone()]).collect_vec();

    for (_, (lhs, rhs)) in pairings.into_iter() {
        let left = circuits.iter().position(|c| c.contains(lhs));
        let right = circuits.iter().position(|c| c.contains(rhs));

        match (left, right) {
            (Some(l), None) => {
                circuits[l].push(rhs.clone());
            }
            (None, Some(r)) => {
                circuits[r].push(lhs.clone());
            }
            (Some(l), Some(r)) => {
                if l != r {
                    let other = circuits[r].clone();
                    circuits[l].extend(other);
                    circuits.remove(r);
                    if circuits.len() == 1 {
                        last_merged_pair = Some((lhs.clone(), rhs.clone()));
                    }
                }
            }
            (None, None) => {
                circuits.push(vec![lhs.clone(), rhs.clone()]);
            }
        }
    }

    if let Some((l, r)) = last_merged_pair {
        l.x as u64 * r.x as u64
    } else {
        0
    }
}

fn main() {
    let junctions = parse(include_str!("input.txt"));
    let result = process_part1(&junctions, 1000, 3);
    println!("PART 1: {}", result);
    let result = process_part2(&junctions);
    println!("PART 2: {}", result);
}

#[cfg(test)]
mod tests {
    use crate::{parse, process_part1, process_part2};

    const INPUT: &str = r#"
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
    "#;

    #[test]
    fn test_parse() {
        let junctions = parse(INPUT);
        assert_eq!(20, junctions.len());
    }

    #[test]
    fn test_part1() {
        let junctions = parse(INPUT);
        assert_eq!(40, process_part1(&junctions, 10, 3));
    }

    #[test]
    fn test_part2() {
        let junctions = parse(INPUT);
        assert_eq!(25272, process_part2(&junctions));
    }
}
