use std::collections::HashSet;

/// Parse text input, atm not using any dedicated parser.
fn parse(input: &str) -> Vec<(i64, i64)> {
    input
        .trim()
        .split_terminator(',')
        .map(|part| {
            let (left, right) = part.trim().split_once("-").expect("Failed to split part");
            let left = left.parse::<i64>().expect("Failed to parse left");
            let right = right.parse::<i64>().expect("Failed to parse right");
            (left, right)
        })
        .collect::<Vec<_>>()
}

/// Calculate the sum of all found invalid ids.
fn solve_part1(ids: &[(i64, i64)]) -> i64 {
    ids.iter()
        .map(|(l, r)| {
            let invalid_ids = find_invalid_ids(*l, *r, 2);
            invalid_ids.iter().sum::<i64>()
        })
        .sum::<i64>()
}

fn solve_part2(ids: &[(i64, i64)]) -> i64 {
    ids.iter()
        .map(|(l, r)| {
            let length = r.to_string().len();

            let mut invalid_ids = HashSet::new();
            for i in 2..=length {
                let set = find_invalid_ids(*l, *r, i);
                for entry in set {
                    invalid_ids.insert(entry);
                }
            }

            invalid_ids.iter().sum::<i64>()
        })
        .sum::<i64>()
}

/// Find invalid ids in a given range.
fn find_invalid_ids(left: i64, right: i64, divisor: usize) -> Vec<i64> {
    let mut result = Vec::new();

    'outer: for x in left..=right {
        let number = x.to_string();
        let size = number.len() / divisor;
        if number.len().rem_euclid(divisor) > 0 {
            continue;
        }

        let first = &number[0..size];
        let mut index = size;
        for _ in 1..divisor {
            let part = &number[index..(index + size)];
            if first.ne(part) {
                continue 'outer;
            };
            index += size;
        }

        result.push(x);
    }

    result
}

fn main() {
    let input = parse(include_str!("input.txt"));
    let result = solve_part1(&input);
    println!("PART 1: {}", result);
    let result = solve_part2(&input);
    println!("PART 2: {}", result);
}

#[cfg(test)]
mod tests {
    use crate::{find_invalid_ids, parse, solve_part1, solve_part2};

    const INPUT: &str = r#"
        11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124
    "#;

    #[test]
    fn check_part1() {
        let input = parse(INPUT);
        assert_eq!(1227775554, solve_part1(&input));
    }

    #[test]
    fn test_find_invalid_ids() {
        assert_eq!(vec![11, 22], find_invalid_ids(11, 22, 2));
    }

    #[test]
    fn check_part2() {
        let input = parse(INPUT);
        assert_eq!(4174379265, solve_part2(&input));
    }
}
