type Id = u64;

#[derive(Debug, Clone)]
struct Range {
    min: Id,
    max: Id,
}

impl Range {
    pub fn new(min: Id, max: Id) -> Self {
        debug_assert!(min <= max);
        Self { min, max }
    }

    pub fn contains(&self, value: Id) -> bool {
        self.min <= value && value <= self.max
    }

    pub fn overlaps(&self, other: &Range) -> bool {
        self.contains(other.min) || self.contains(other.max)
    }

    pub fn merge(&mut self, other: &Range) {
        debug_assert!(self.overlaps(other));
        self.min = std::cmp::min(self.min, other.min);
        self.max = std::cmp::max(self.max, other.max);
    }

    /// Returns count of Ids in range (inclusive)
    pub fn count(&self) -> u64 {
        self.max - self.min + 1
    }
}

/// Parses ranges and ingredients
///
/// NOTE: ranges can consist of a single element, e.g. 11-11
fn parse_input(input: &str) -> (Vec<Range>, Vec<Id>) {
    let (ranges, ingredients) = input.split_once("\n\n").expect("Failed to parse input");

    let mut ranges = ranges
        .lines()
        .filter_map(|line| line.trim().split_once('-'))
        .map(|(l, r)| Range::new(l.parse::<Id>().unwrap(), r.parse::<Id>().unwrap()))
        .collect::<Vec<_>>();

    // ranges can overlap
    ranges.sort_by(|range, other| range.min.cmp(&other.min));

    let ingredients = ingredients
        .lines()
        .map(|line| line.trim().parse::<Id>().unwrap())
        .collect::<Vec<_>>();

    (ranges, ingredients)
}

fn process_part1((ranges, ingredients): &(Vec<Range>, Vec<Id>)) -> usize {
    ingredients
        .iter()
        .filter(|id| ranges.iter().find(|range| range.contains(**id)).is_some())
        .count()
}

fn process_part2((ranges, _): &(Vec<Range>, Vec<Id>)) -> usize {
    let mut combined: Vec<Range> = Vec::new();

    // compare with all existing ranges
    for range in ranges {
        if let Some(combined) = combined
            .iter_mut()
            .find(|other| other.overlaps(range))
        {
            combined.merge(range);
        } else {
            combined.push(range.clone());
        }
    }

    combined.iter().map(|range| range.count()).sum::<u64>() as usize
}

fn main() {
    let input = parse_input(include_str!("input.txt"));
    let result = process_part1(&input);
    println!("PART 1: {}", result);
    let result = process_part2(&input);
    println!("PART 2: {}", result);
}

#[cfg(test)]
mod tests {
    use crate::*;

    const INPUT: &str = r#"3-5
10-14
16-20
12-18

1
5
8
11
17
32"#;

    #[test]
    fn test_parse_input() {
        let (ranges, ingredients) = parse_input(INPUT);
        assert_eq!(4, ranges.len());
        assert_eq!(6, ingredients.len());
    }

    #[test]
    fn test_part1() {
        assert_eq!(3, process_part1(&parse_input(INPUT)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(14, process_part2(&parse_input(INPUT)));
    }

    #[test]
    fn test_range_merge() {
        assert!(Range::new(1, 4).overlaps(&Range::new(4, 7)));
    }
}
