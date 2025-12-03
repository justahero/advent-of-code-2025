fn parse_input(input: &str) -> Vec<String> {
    input
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
}

/// Find the highest jolt, pair of combined digits.
fn calculate_jolt(bank: &str) -> u32 {
    let digits = bank
        .chars()
        .filter_map(|digit| digit.to_digit(10))
        .collect::<Vec<_>>();
    let size = digits.len();

    let mut first = digits[0];
    let mut second = digits[1];

    for i in 1..(size - 1) {
        // find highest first digit
        if digits[i] > first {
            first = digits[i];
            second = digits[i + 1];
        }

        // find highest second digit in remaining range
        for j in i + 1..size {
            if digits[j] > second {
                second = digits[j];
            }
        }
    }

    first * 10 + second
}

/// For each bank / string, get all digits and find the two highest ones
fn process_part1(banks: &[String]) -> u32 {
    banks
        .iter()
        .fold(0, |total, bank| total + calculate_jolt(bank.as_str()))
}

fn main() {
    let input = parse_input(include_str!("input.txt"));
    let result = process_part1(&input);
    println!("PART 1: {}", result);
}

#[cfg(test)]
mod tests {
    use crate::{calculate_jolt, parse_input, process_part1};

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
    fn test_calculate_jolt() {
        assert_eq!(89, calculate_jolt("811111111111119"));
        assert_eq!(
            99,
            calculate_jolt(
                "2215452689925244273244333436189317446384838478525478824435233342352236255624326767355438753493222423"
            )
        );
    }
}
