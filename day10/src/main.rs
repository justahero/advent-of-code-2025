#![allow(dead_code)]

use std::ops::Shl;

use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::{char, complete::space1},
    multi::{fold_many1, separated_list1},
    sequence::delimited,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct BitVec(u16);

impl BitVec {
    pub fn new(bits: &[u8]) -> Self {
        let bits = bits.iter().fold(0u16, |mut bits, index| {
            debug_assert!(*index < 16);
            bits |= 1u16.shl(index);
            bits
        });
        Self(bits)
    }

    pub fn bits(&self) -> u16 {
        self.0
    }

    pub fn toggle(&self, rhs: &BitVec) -> Self {
        Self(self.0 ^ rhs.0)
    }
}

struct Machine {
    // [.##.]
    lights: BitVec,
    // (3) (1,3) (2) (2,3) (0,2) (0,1)
    buttons: Vec<BitVec>,
    // joltage requirements
    joltage: Vec<u8>,
}

impl Machine {
    pub fn new(lights: Vec<u8>, buttons: Vec<BitVec>, joltage: Vec<u8>) -> Self {
        Self {
            lights: BitVec::new(&lights),
            buttons,
            joltage,
        }
    }

    /// Determine the number of fewest presses to match the indicator lights, e.g. `[.##.]`.
    ///
    /// Initially all lights are off, a sequence with the fewest pressed buttons is calculated.
    pub fn fewest_presses(&self) -> u32 {
        0
    }
}

///
fn recursive(target: &BitVec, lights: BitVec, buttons: &[BitVec], level: u32) -> bool {
    if target == &lights {
        return true;
    }

    for button in buttons.iter() {
        if recursive(target, lights.toggle(button), buttons, level + 1) {
            return true;
        }
    }
    false
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

    let buttons = buttons.iter().map(|b| BitVec::new(&b)).collect::<Vec<_>>();
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

fn parse_buttons_list(input: &str) -> IResult<&str, Vec<Vec<u8>>> {
    separated_list1(space1, parse_buttons).parse(input)
}

fn parse_buttons(input: &str) -> IResult<&str, Vec<u8>> {
    delimited(
        tag("("),
        separated_list1(tag(","), nom::character::complete::u8),
        tag(")"),
    )
    .parse(input)
}

fn parse_joltage(input: &str) -> IResult<&str, Vec<u8>> {
    delimited(
        tag("{"),
        separated_list1(tag(","), nom::character::complete::u8),
        tag("}"),
    )
    .parse(input)
}

fn process_part1(machines: &[Machine]) -> u32 {
    machines.iter().map(Machine::fewest_presses).sum::<u32>()
}

fn main() {
    let machines = parse_input(include_str!("input.txt"));
    let result = process_part1(&machines);
    println!("PART 1: {}", result);
    // TODO
}

#[cfg(test)]
mod tests {
    use crate::{BitVec, parse_buttons_list, parse_input, parse_joltage, parse_lights};

    const INPUT: &str = r#"
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
    "#;

    #[test]
    fn test_buttons_constructor() {
        assert_eq!(0b1001, BitVec::new(&[0, 3]).0);
    }

    #[test]
    fn test_buttons_toggle() {
        let expected = BitVec::new(&[0, 3]);
        let lights = BitVec::new(&[1, 2]);
        assert_eq!(expected, lights.toggle(&BitVec::new(&[0, 1, 2, 3])));
    }

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
