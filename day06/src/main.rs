use anyhow::anyhow;

#[derive(Debug)]
struct Equation {
    numbers: Vec<u64>,
    op: u8,
}

#[derive(Debug)]
struct Block {
    pub matrix: Vec<Vec<u8>>,
}

impl Block {
    /// Returns the operator, expected to be in the last row
    pub fn op(&self) -> Option<u8> {
        let length = self.matrix.len();
        self.matrix[length - 1]
            .iter()
            .find(|byte| **byte != SPACE)
            .cloned()
    }

    /// Parse all values from rows and returns them.
    pub fn row_values(&self) -> anyhow::Result<Vec<u64>> {
        let mut result = Vec::new();
        let num_columns = self.matrix.len() - 1;
        for index in 0..num_columns {
            let value = str::from_utf8(&self.matrix[index])?.trim().parse::<u64>()?;
            result.push(value);
        }
        Ok(result)
    }

    /// Parse all values from columns and returns them.
    pub fn col_values(&self) -> anyhow::Result<Vec<u64>> {
        println!("MATRIX: {:?}", self.matrix);
        let rows = self.matrix.len();
        let mut result = Vec::new();

        for col in 0..self.matrix[0].len() {
            let value = (0..rows - 1)
                .map(|row| self.matrix[row][col])
                .collect::<Vec<_>>();
            let value = str::from_utf8(&value)?.trim().parse::<u64>()?;
            println!("  value: {}", value);
            result.push(value);
        }

        Ok(result)
    }
}

impl Equation {
    pub fn calculate_total(&self) -> u64 {
        match self.op {
            b'*' => self.numbers.iter().product(),
            b'+' => self.numbers.iter().sum(),
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

fn parse_ops(line: &str) -> anyhow::Result<Vec<u8>> {
    Ok(line
        .split_ascii_whitespace()
        .map(|item| match item {
            "*" => Ok(b'*'),
            "+" => Ok(b'+'),
            _ => Err(anyhow!("Failed to parse char")),
        })
        .collect::<Result<Vec<_>, _>>()?)
}

// TODO: remove this function, merge with "parse_second"
fn parse_input(input: &str) -> Vec<Equation> {
    // read in all numbers / tokens
    let mut numbers: Vec<Vec<u64>> = Vec::new();
    let mut operations: Vec<u8> = Vec::new();

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

const SPACE: u8 = b' ';

/// Ignore error handling
fn parse_second(input: &str) -> Vec<Block> {
    let mut result = Vec::new();
    let lines = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.as_bytes().to_vec())
        .collect::<Vec<_>>();
    let rows = lines.len();

    // TODO: refactor this block
    // group all connected columns, keep white spaces
    let mut col = 0;
    for new_col in 0..lines[0].len() {
        if (0..rows).all(|row| lines[row][new_col] == SPACE) {
            let matrix = (0..rows)
                .into_iter()
                .map(|row| lines[row][col..new_col].to_vec())
                .collect::<Vec<_>>();
            dbg!(&matrix);
            let matrix = Block { matrix };
            result.push(matrix);
            col = new_col + 1;
        }
    }

    let matrix = (0..rows)
        .into_iter()
        .map(|row| lines[row][col..lines[0].len()].to_vec())
        .collect::<Vec<_>>();
    let matrix = Block { matrix };
    result.push(matrix);

    result
}

fn convert_part2(blocks: Vec<Block>) -> Vec<Equation> {
    let mut equations = Vec::new();

    // Convert each block into the appropriate equation
    for block in blocks {
        let numbers = block.col_values().expect("Failed to parse values");
        let op = block.op().unwrap();
        equations.push(Equation { numbers, op });
    }

    equations
}

fn process_part2(input: &str) -> u64 {
    let blocks = parse_second(input);
    let equations = convert_part2(blocks);
    equations.iter().map(Equation::calculate_total).sum::<u64>()
}

fn main() {
    let input = parse_input(include_str!("input.txt"));
    let result = process_part1(&input);
    println!("PART 1: {}", result);
}

#[cfg(test)]
mod tests {
    use crate::{
        parse_input, parse_numbers, parse_ops, parse_second, process_part1, process_part2,
    };

    const INPUT: &str = r#"
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  "#;

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

    #[test]
    fn test_part2() {
        let result = process_part2(INPUT);
        assert_eq!(3263827, result);
    }

    #[test]
    fn test_parse_blocks() {
        let blocks = parse_second(INPUT);
        assert_eq!(4, blocks.len());

        let block = &blocks[0];
        let rows = block.row_values().unwrap();
        assert_eq!(vec![123, 45, 6], rows);

        let cols = block.col_values().unwrap();
        assert_eq!(vec![1, 24, 356], cols);
    }
}
