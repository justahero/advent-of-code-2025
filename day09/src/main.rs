#![allow(dead_code)]
#![allow(unused_variables)]

use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
struct Pos {
    x: i64,
    y: i64,
}

impl Pos {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub fn area(&self, rhs: &Pos) -> i64 {
        ((self.x - rhs.x).abs() + 1) * ((self.y - rhs.y).abs() + 1)
    }
}

/// An edge connecting two red tiles, all intermediate tiles are green.
#[derive(Debug, Clone)]
struct Edge {
    pub start: Pos,
    pub end: Pos,
}

impl Edge {
    /// Creates new edge
    pub fn new(start: Pos, end: Pos) -> Self {
        // Edges require their positions are on the same axis, either x or y
        debug_assert!(start.x == end.x || start.y == end.y);
        Self { start, end }
    }

    /// Returns true if the position is considered "inside"
    pub fn inside(&self, pos: &Pos) -> bool {
        true
    }
}

fn parse_input(input: &str) -> Vec<Pos> {
    input
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .filter_map(|line| line.split_once(','))
        .map(|(l, r)| (l.parse::<i64>().unwrap(), r.parse::<i64>().unwrap()))
        .map(|(x, y)| Pos::new(x, y))
        .collect::<Vec<_>>()
}

fn process_part1(positions: &[Pos]) -> u64 {
    let mut max_area: u64 = 0;
    for (l, r) in positions.iter().tuple_combinations() {
        max_area = std::cmp::max(max_area, l.area(&r) as u64);
    }
    max_area
}

/// Returns the set of edges spanning an area. Positions are red tiles, all other tiles of the area
/// are green tiles.
fn get_edges(positions: &[Pos]) -> Vec<Edge> {
    let mut edges = positions
        .iter()
        .tuple_windows()
        .map(|(l, r)| Edge::new(*l, *r))
        .collect::<Vec<_>>();
    edges.push(Edge::new(positions[positions.len() - 1], positions[0]));
    edges
}

/// Checks if the pair of positions is completely inside the area outlined by edges
fn inside_area(edges: &[Edge], left: &Pos, right: &Pos) -> bool {
    false
}

/// Given is the list of red tiles.
///
/// Green tiles are the edges between red tiles on the same axis, the boundary connecting all red tiles?
fn process_part2(positions: &[Pos]) -> u64 {
    let edges = get_edges(positions);

    let mut max_area: u64 = 0;
    for (l, r) in positions.iter().tuple_combinations() {}

    max_area
}

fn main() {
    let positions = parse_input(include_str!("input.txt"));
    let result = process_part1(&positions);
    println!("PART 1: {}", result);
    let result = process_part2(&positions);
    println!("PART 2: {}", result);
}

#[cfg(test)]
mod tests {
    use crate::{Pos, get_edges, inside_area, parse_input, process_part1, process_part2};

    const INPUT: &str = r#"
        7,1
        11,1
        11,7
        9,7
        9,5
        2,5
        2,3
        7,3
    "#;

    #[test]
    fn test_parse() {
        let positions = parse_input(INPUT);
        assert_eq!(8, positions.len());
    }

    #[test]
    fn test_pos_area() {
        assert_eq!(50, Pos::new(2, 5).area(&Pos::new(11, 1)));
    }

    #[test]
    fn test_part1() {
        let positions = parse_input(INPUT);
        assert_eq!(50, process_part1(&positions));
    }

    #[test]
    fn test_part2() {
        let positions = parse_input(INPUT);
        assert_eq!(24, process_part2(&positions));
    }

    #[test]
    fn test_inside_area() {
        let positions = parse_input(INPUT);
        let edges = get_edges(&positions);
        assert!(inside_area(
            &edges,
            &Pos { x: 7, y: 3 },
            &Pos { x: 11, y: 1 }
        ));
        assert!(inside_area(
            &edges,
            &Pos { x: 9, y: 7 },
            &Pos { x: 9, y: 5 }
        ));
        assert!(inside_area(
            &edges,
            &Pos { x: 9, y: 5 },
            &Pos { x: 2, y: 3 }
        ));
    }
}
