#![allow(unused_variables)]
#![allow(unused_mut)]

fn parse_input(input: &str) -> Vec<String> {
    input
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
}

fn calculate_jolt(bank: &str, num_batteries: usize) -> u64 {
    let digits = bank.chars().collect::<Vec<_>>();
    let size = digits.len();

    // keep list of indices
    let mut indices: Vec<usize> = (0..num_batteries).into_iter().collect();

    // for each battery slot, determine the highest battery from available range
    for pos in 0..num_batteries {
        let mut battery = digits[indices[pos]];
        let end = size - num_batteries + pos;

        for index in (indices[pos] + 1)..=end {
            let new_battery = digits[index];
            if new_battery > battery {
                battery = new_battery;
                for (i, pos) in (pos..num_batteries).enumerate() {
                    indices[pos] = index + i;
                }
            }
        }
    }

    indices
        .into_iter()
        .map(|index| digits[index])
        .collect::<String>()
        .parse::<u64>()
        .expect("Failed to produce jolt")
}

fn process(banks: &[String], batteries: usize) -> u64 {
    banks.iter().fold(0, |total, bank| {
        total + calculate_jolt(bank.as_str(), batteries)
    })
}

/// For each bank / string, get all digits and find the two highest ones
fn process_part1(banks: &[String]) -> u64 {
    process(banks, 2)
}

fn process_part2(banks: &[String]) -> u64 {
    process(banks, 12)
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
    use crate::{calculate_jolt, parse_input, process_part1, process_part2};

    const INPUT: &str = r#"
        987654321111111
        811111111111119
        234234234234278
        818181911112111
    "#;

    #[test]
    fn check_part1() {
        let input = parse_input(INPUT);
        assert_eq!(357, process_part1(&input));
    }

    #[test]
    fn check_part2() {
        let input = parse_input(INPUT);
        assert_eq!(3121910778619, process_part2(&input));
    }

    #[test]
    fn test_calculate_jolt() {
        assert_eq!(89, calculate_jolt("811111111111119", 2));
        assert_eq!(888911112111, calculate_jolt("818181911112111", 12));
        assert_eq!(987654321111, calculate_jolt("987654321111111", 12));
        assert_eq!(811111111119, calculate_jolt("811111111111119", 12));
        assert_eq!(434234234278, calculate_jolt("234234234234278", 12));
    }

    #[test]
    fn test_calculate_extra_jolt() {
        assert_eq!(891, calculate_jolt("8111191", 3));
        assert_eq!(8191, calculate_jolt("8111191", 4));
    }
}
