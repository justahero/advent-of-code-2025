#![allow(dead_code)]

use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::{char, complete::space1},
    multi::{fold_many1, separated_list1},
    sequence::delimited,
};

struct Machine {
    // [.##.]
    lights: Vec<u8>,
    // (3) (1,3) (2) (2,3) (0,2) (0,1)
    buttons: Vec<Vec<u16>>,
    // joltage requirements
    joltage: Vec<u16>,
}

impl Machine {
    pub fn new(lights: Vec<u8>, buttons: Vec<Vec<u16>>, joltage: Vec<u16>) -> Self {
        Self {
            lights,
            buttons,
            joltage,
        }
    }
}

fn parse_input(input: &str) -> Vec<Machine> {
    input
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(parse_machine)
        .collect::<Vec<_>>()
}

fn parse_machine(line: &str) -> Machine {
    let (_, (lights, _, buttons, _, joltage)) = (
        parse_lights,
        space1,
        parse_buttons_list,
        space1,
        parse_joltage,
    )
        .parse(line)
        .expect("Failed to parse input");

    Machine::new(lights, buttons, joltage)
}

fn parse_lights(input: &str) -> IResult<&str, Vec<u8>> {
    delimited(
        tag("["),
        fold_many1(
            alt((char('.'), char('#'))),
            Vec::new,
            |mut acc: Vec<_>, item| {
                acc.push(match item {
                    '.' => 0,
                    _ => 1,
                });
                acc
            },
        ),
        tag("]"),
    )
    .parse(input)
}

fn parse_buttons_list(input: &str) -> IResult<&str, Vec<Vec<u16>>> {
    separated_list1(space1, parse_buttons).parse(input)
}

fn parse_buttons(input: &str) -> IResult<&str, Vec<u16>> {
    delimited(
        tag("("),
        separated_list1(tag(","), nom::character::complete::u16),
        tag(")"),
    )
    .parse(input)
}

fn parse_joltage(input: &str) -> IResult<&str, Vec<u16>> {
    delimited(
        tag("{"),
        separated_list1(tag(","), nom::character::complete::u16),
        tag("}"),
    )
    .parse(input)
}

fn process_part1(machines: &[Machine]) -> u64 {
    0
}

fn main() {
    let machines = parse_input(include_str!("input.txt"));
    let result = process_part1(&machines);
    // TODO
}

#[cfg(test)]
mod tests {
    use crate::{parse_buttons_list, parse_input, parse_joltage, parse_lights};

    const INPUT: &str = r#"
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
    "#;

    #[test]
    fn test_parse_lights() {
        assert_eq!(parse_lights("[.##.]").unwrap(), ("", vec![0, 1, 1, 0]));
    }

    #[test]
    fn test_parse_buttons_list() {
        let input: &str = "(3) (1,3) (2)";
        assert_eq!(
            parse_buttons_list(input).unwrap(),
            ("", vec![vec![3], vec![1, 3], vec![2]])
        );
    }

    #[test]
    fn test_parse_joltage() {
        assert_eq!(parse_joltage("{1,2,3}").unwrap(), ("", vec![1, 2, 3]));
    }

    #[test]
    fn test_parse() {
        let machines = parse_input(INPUT);
        assert_eq!(3, machines.len());
    }

    #[test]
    fn test_part1() {
        // TODO
    }
}
