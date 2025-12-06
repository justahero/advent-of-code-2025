use anyhow::anyhow;

struct Equation {
    numbers: Vec<u64>,
    op: char,
}

impl Equation {
    pub fn calculate_total(&self) -> u64 {
        match self.op {
            '*' => self.numbers.iter().product(),
            '+' => self.numbers.iter().sum(),
            _ => panic!("what??"),
        }
    }
}

fn parse_numbers(line: &str) -> anyhow::Result<Vec<u64>> {
    Ok(line
        .split_ascii_whitespace()
        .map(|item| item.parse::<u64>())
        .collect::<Result<Vec<_>, _>>()?)
}

fn parse_ops(line: &str) -> anyhow::Result<Vec<char>> {
    Ok(line
        .split_ascii_whitespace()
        .map(|item| match item {
            "*" => Ok('*'),
            "+" => Ok('+'),
            _ => Err(anyhow!("Failed to parse char")),
        })
        .collect::<Result<Vec<_>, _>>()?)
}

fn parse_input(input: &str) -> Vec<Equation> {
    // read in all numbers / tokens
    let mut numbers: Vec<Vec<u64>> = Vec::new();
    let mut operations: Vec<char> = Vec::new();

    for line in input.lines().filter(|line| !line.is_empty()) {
        if let Ok(result) = parse_numbers(line) {
            numbers.push(result);
        } else if let Ok(result) = parse_ops(line) {
            operations = result;
        }
    }

    // Transpose columns / rows
    let mut result = Vec::new();
    for (index, op) in operations.iter().enumerate() {
        let numbers = numbers.iter().map(|row| row[index]).collect::<Vec<_>>();
        result.push(Equation { numbers, op: *op });
    }
    result
}

fn process_part1(equations: &[Equation]) -> u64 {
    equations.iter().map(Equation::calculate_total).sum::<u64>()
}

fn main() {
    let input = parse_input(include_str!("input.txt"));
    let result = process_part1(&input);
    println!("PART 1: {}", result);
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, parse_numbers, parse_ops, process_part1};

    const INPUT: &str = r#"
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +"#;

    #[test]
    fn test_parse_methods() {
        assert!(parse_numbers("123 328  51 64").is_ok());
        assert!(parse_numbers("*   +   *   +").is_err());
        assert!(parse_ops("123 328  51 64").is_err());
        assert!(parse_ops("*   +   *   +").is_ok());
    }

    #[test]
    fn test_part1() {
        let equations = parse_input(INPUT);
        assert_eq!(4277556, process_part1(&equations));
    }
}
