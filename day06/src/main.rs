const SPACE: u8 = b' ';

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
        self.matrix[self.num_rows()]
            .iter()
            .find(|byte| **byte != SPACE)
            .cloned()
    }

    /// Parse all values from rows and returns them.
    pub fn row_values(&self) -> anyhow::Result<Vec<u64>> {
        (0..self.num_rows())
            .map(|row| self.row(row))
            .collect::<anyhow::Result<Vec<_>>>()
    }

    /// Parse all values from columns and returns them.
    pub fn col_values(&self) -> anyhow::Result<Vec<u64>> {
        (0..self.num_columns())
            .map(|col| self.col(col))
            .collect::<anyhow::Result<Vec<_>>>()
    }

    fn row(&self, row: usize) -> anyhow::Result<u64> {
        Ok(str::from_utf8(&self.matrix[row])?.trim().parse::<u64>()?)
    }

    fn col(&self, column: usize) -> anyhow::Result<u64> {
        let value = (0..self.num_rows())
            .map(|row| self.matrix[row][column])
            .collect::<Vec<_>>();
        Ok(str::from_utf8(&value)?.trim().parse::<u64>()?)
    }

    fn num_columns(&self) -> usize {
        self.matrix[0].len()
    }

    /// Returns the number of rows that contain values, except the operator
    fn num_rows(&self) -> usize {
        self.matrix.len() - 1
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

fn process_part1(blocks: &[Block]) -> u64 {
    let equations = convert_part1(blocks);
    equations.iter().map(Equation::calculate_total).sum::<u64>()
}

/// Ignore error handling
fn parse_blocks(input: &str) -> Vec<Block> {
    let mut result = Vec::new();
    let lines = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.as_bytes())
        .collect::<Vec<_>>();
    let rows = lines.len();

    let mut col = 0;
    for new_col in 0..lines[0].len() {
        let new_col = if (0..rows).all(|row| lines[row][new_col] == SPACE) {
            new_col
        } else if new_col == lines[0].len() - 1 {
            lines[0].len()
        } else {
            continue;
        };

        let matrix = (0..rows)
            .map(|row| lines[row][col..new_col].to_vec())
            .collect::<Vec<_>>();

        result.push(Block { matrix });
        col = new_col + 1;
    }

    result
}

fn convert_part1(blocks: &[Block]) -> Vec<Equation> {
    let mut equations = Vec::new();

    for block in blocks {
        let numbers = block.row_values().expect("Failed to parse values");
        let op = block.op().unwrap();
        equations.push(Equation { numbers, op });
    }

    equations
}

fn convert_part2(blocks: &[Block]) -> Vec<Equation> {
    let mut equations = Vec::new();

    for block in blocks {
        let numbers = block.col_values().expect("Failed to parse values");
        let op = block.op().unwrap();
        equations.push(Equation { numbers, op });
    }

    equations
}

fn process_part2(blocks: &[Block]) -> u64 {
    let equations = convert_part2(blocks);
    equations.iter().map(Equation::calculate_total).sum::<u64>()
}

fn main() {
    let blocks = parse_blocks(include_str!("input.txt"));
    let result = process_part1(&blocks);
    println!("PART 1: {}", result);
    let result = process_part2(&blocks);
    println!("PART 2: {}", result);
}

#[cfg(test)]
mod tests {
    use crate::{parse_blocks, process_part1, process_part2};

    const INPUT: &str = r#"
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  "#;

    #[test]
    fn test_part1() {
        let blocks = parse_blocks(INPUT);
        assert_eq!(4277556, process_part1(&blocks));
    }

    #[test]
    fn test_part2() {
        let blocks = parse_blocks(INPUT);
        let result = process_part2(&blocks);
        assert_eq!(3263827, result);
    }

    #[test]
    fn test_parse_blocks() {
        let blocks = parse_blocks(INPUT);
        assert_eq!(4, blocks.len());

        let block = &blocks[0];
        let rows = block.row_values().unwrap();
        assert_eq!(vec![123, 45, 6], rows);

        let cols = block.col_values().unwrap();
        assert_eq!(vec![1, 24, 356], cols);
    }
}
