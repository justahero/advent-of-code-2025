/// Parse input, split lines. Each number in the line results in a positive (R) or negative (L)
/// value.
fn parse_input(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(String::from)
        .map(|mut s| {
            let direction = s.remove(0);
            let number = s
                .parse::<i32>()
                .expect(&format!("Failed to parse line: '{}'", s));
            if direction == 'R' { number } else { -number }
        })
        .collect::<Vec<_>>()
}

/// Apply rotations, count the number of times the dial is exactly at zero.
fn rotate_list(start_dial: i32, rotations: Vec<i32>) -> i32 {
    let mut zeroes = 0;

    let mut current = start_dial;
    for rotation in rotations {
        current = (current + rotation) % 100;
        if current == 0 {
            zeroes += 1;
        }
    }

    zeroes
}

fn main() {
    let rotations = parse_input(include_str!("input.txt"));
    let result = rotate_list(50, rotations);
    println!("PASSWORD: {}", result);
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, rotate_list};

    #[test]
    fn example_works() {
        let input = "
            L68
            L30
            R48
            L5
            R60
            L55
            L1
            L99
            R14
            L82
        ";
        let rotations = parse_input(input);
        assert_eq!(3, rotate_list(50, rotations));
    }
}
