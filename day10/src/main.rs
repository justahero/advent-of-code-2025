#![allow(dead_code)]

use std::{collections::VecDeque, ops::Shl};

use good_lp::{Expression, Solution, SolverModel, Variable, variable, variables};
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
    /// Converts a list of indices to bits.
    pub fn new(bits: &[u8]) -> Self {
        let bits = bits.iter().fold(0u16, |mut bits, index| {
            debug_assert!(*index < 16);
            bits |= 1u16.shl(index);
            bits
        });
        Self(bits)
    }

    pub fn bit_set(&self, index: u32) -> bool {
        self.0 & 1u16.shl(index) > 0
    }

    pub fn bits(&self) -> u16 {
        self.0
    }

    pub fn toggle(&self, rhs: &BitVec) -> Self {
        Self(self.0 ^ rhs.0)
    }
}

impl std::ops::BitXor for BitVec {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        BitVec(self.0 ^ rhs.0)
    }
}

impl std::fmt::Display for BitVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#08b}", self.0)
    }
}

struct Machine {
    // [.##.]
    lights: BitVec,
    // (3) (1,3) (2) (2,3) (0,2) (0,1)
    buttons: Vec<BitVec>,
    // joltage requirements
    joltage: Vec<u16>,
}

impl Machine {
    pub fn new(lights: Vec<u8>, buttons: Vec<Vec<u8>>, joltage: Vec<u16>) -> Self {
        let lights = BitVec::new(&lights);
        let buttons = buttons.iter().map(|b| BitVec::new(&b)).collect::<Vec<_>>();

        Self {
            lights,
            buttons,
            joltage,
        }
    }

    const INITIAL_LIGHTS: BitVec = BitVec(0);

    /// Determine the number of fewest presses to match the indicator lights, e.g. `[.##.]`.
    pub fn light_presses(&self) -> u32 {
        let mut queue: VecDeque<(u32, BitVec)> = VecDeque::from([(0, Self::INITIAL_LIGHTS)]);

        loop {
            let (level, lights) = queue.pop_front().expect("Failed to get first item");

            if lights == self.lights {
                return level;
            }

            // otherwise press each buttons combination and store to queue.
            for button in self.buttons.iter() {
                queue.push_back((level + 1, lights ^ *button));
            }
        }
    }

    /// For reference https://github.com/NickyMeuleman/scrapyard/blob/main/advent_of_code/2025/solutions/src/day_10.rs
    pub fn joltage_presses(&self) -> u32 {
        let mut vars = variables!();

        // Map buttons to variables
        let presses: Vec<Variable> = (0..self.buttons.len())
            .map(|_| vars.add(variable().min(0).integer()))
            .collect();

        // Minimize total presses
        let total_presses: Expression = presses.iter().sum();
        let mut problem = vars.minimise(total_presses).using(good_lp::default_solver);

        // Set all constraints
        for (index, &target) in self.joltage.iter().enumerate() {
            let mut expression = Expression::from(0.0);

            for (button_index, button) in self.buttons.iter().enumerate() {
                if button.bit_set(index as u32) {
                    expression += presses[button_index]
                }
            }

            problem.add_constraint(expression.eq(target));
        }

        let solution = problem.solve().expect("Failed to find solution");

        presses
            .iter()
            .map(|v| solution.value(*v).round() as u32)
            .sum::<u32>()
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

    // Convert list of bits into indices
    let lights = lights
        .iter()
        .enumerate()
        .filter(|v| *v.1 > 0)
        .map(|(index, _)| index as u8)
        .collect::<Vec<_>>();

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

fn parse_joltage(input: &str) -> IResult<&str, Vec<u16>> {
    delimited(
        tag("{"),
        separated_list1(tag(","), nom::character::complete::u16),
        tag("}"),
    )
    .parse(input)
}

fn process_part1(machines: &[Machine]) -> u32 {
    machines.iter().map(Machine::light_presses).sum::<u32>()
}

fn process_part2(machines: &[Machine]) -> u32 {
    machines.iter().map(Machine::joltage_presses).sum::<u32>()
}

fn main() {
    let machines = parse_input(include_str!("input.txt"));
    let result = process_part1(&machines);
    println!("PART 1: {}", result);
    let result = process_part2(&machines);
    println!("PART 2: {}", result);
}

#[cfg(test)]
mod tests {
    use crate::{
        parse_buttons_list, parse_input, parse_joltage, parse_lights, process_part1, process_part2,
    };

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
    fn test_single_machine_lights() {
        let input: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
        let machines = parse_input(input);
        assert_eq!(2, machines[0].light_presses());
    }

    #[test]
    fn test_single_machine_joltage() {
        let input: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
        let machines = parse_input(input);
        assert_eq!(10, machines[0].joltage_presses());
    }

    #[test]
    fn test_part1() {
        let machines = parse_input(INPUT);
        assert_eq!(7, process_part1(&machines));
    }

    #[test]
    fn test_part2() {
        let machines = parse_input(INPUT);
        assert_eq!(33, process_part2(&machines));
    }

    #[test]
    fn test_part2_first_machine_from_input() {
        let input = "[.#.#] (0,2,3) (1,3) (2,3) (0,1,2) (0) {31,4,31,29}";
        let machines = parse_input(input);
        assert_eq!(32, machines[0].joltage_presses());
    }
}
