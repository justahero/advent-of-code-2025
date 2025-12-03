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

/// Count the number of times the dial is exactly at zero.
fn rotate_first(start_dial: i32, rotations: &[i32]) -> i32 {
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

/// Count the number of times the dial passes zero and stays on zero.
fn rotate_second(start_dial: i32, rotations: &[i32]) -> i32 {
    let mut zeroes = 0;

    let mut current = start_dial;
    for rotation in rotations {
        let updated = current + rotation;
        if current != 0 && updated <= 0 {
            zeroes += 1;
        }

        current = updated.rem_euclid(100);
        zeroes += (updated / 100).abs();
    }

    zeroes
}

fn main() {
    let rotations = parse_input(include_str!("input.txt"));
    let result = rotate_first(50, &rotations);
    println!("PASSWORD: {}", result);
    let result = rotate_second(50, &rotations);
    println!("PASSWORD: {}", result);
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, rotate_first, rotate_second};

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
        assert_eq!(3, rotate_first(50, &rotations));
        assert_eq!(6, rotate_second(50, &rotations));
    }

    #[test]
    fn check_bounds() {
        assert_eq!(1, rotate_second(50, &[-100]));
        assert_eq!(10, rotate_second(50, &[1000]));
    }
}
