type Id = u64;

fn parse_input(input: &str) -> (Vec<(Id, Id)>, Vec<Id>) {
    let (ranges, ingredients) = input.split_once("\n\n").expect("Failed to parse input");

    let ranges = ranges
        .lines()
        .filter_map(|line| line.trim().split_once('-'))
        .map(|(l, r)| (l.parse::<Id>().unwrap(), r.parse::<Id>().unwrap()))
        .collect::<Vec<_>>();

    let ingredients = ingredients
        .lines()
        .map(|line| line.trim().parse::<Id>().unwrap())
        .collect::<Vec<_>>();

    (ranges, ingredients)
}

fn process_part1((ranges, ingredients): (Vec<(Id, Id)>, Vec<Id>)) -> usize {
    ingredients
        .into_iter()
        .filter(|id| {
            ranges
                .iter()
                .find(|(min, max)| min <= id && id <= max)
                .is_some()
        })
        .count()
}

fn main() {
    let input = parse_input(include_str!("input.txt"));
    let result = process_part1(input);
    println!("PART 1: {}", result);
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, process_part1};

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
        assert_eq!(3, process_part1(parse_input(INPUT)));
    }
}
