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

fn main() {
    let positions = parse_input(include_str!("input.txt"));
    let result = process_part1(&positions);
    println!("PART 1: {}", result);
}

#[cfg(test)]
mod tests {
    use crate::{Pos, parse_input, process_part1};

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
}
