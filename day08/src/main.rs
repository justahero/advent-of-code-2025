#![allow(dead_code)]

use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

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
fn process_part1(junctions: &[Vec3], max_pairs: usize, largest: usize) -> u64 {
    // Generate all pairings, sorted by distance between positions
    let mut pairings: Vec<(u64, Pair)> = Vec::new();
    for l in 0..junctions.len() - 1 {
        let lhs = &junctions[l];
        for r in l + 1..junctions.len() {
            let rhs = &junctions[r];
            let distance = rhs.distance_squared(lhs);
            pairings.push((distance, (l, r)));
        }
    }
    pairings.sort_by(|(left, _), (right, _)| left.cmp(right));

    // Collect all circuits.
    let mut circuits: Vec<Vec<Vec3>> = Vec::new();
    for (_, (l, r)) in pairings.into_iter().take(max_pairs) {
        let lhs = &junctions[l];
        let rhs = &junctions[r];

        if let Some(circuit) = circuits.iter_mut().find(|circuit| circuit.contains(&lhs)) {
            circuit.push(lhs.clone());
        } else if let Some(circuit) = circuits.iter_mut().find(|circuit| circuit.contains(&rhs)) {
            circuit.push(rhs.clone());
        } else {
            circuits.push(vec![lhs.clone(), rhs.clone()]);
        }
    }

    circuits.sort_by(|l, r| r.len().cmp(&l.len()));
    circuits
        .into_iter()
        .take(largest)
        .map(|circuit| circuit.len() as u64)
        .product()
}

fn main() {
    let junctions = parse(include_str!("input.txt"));
    let result = process_part1(&junctions, 1000, 3);
    println!("PART 1: {}", result);
    // 720 too low
}

#[cfg(test)]
mod tests {
    use crate::{parse, process_part1};

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
}
