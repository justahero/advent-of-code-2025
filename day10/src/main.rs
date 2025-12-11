#![allow(dead_code)]

use std::collections::{BTreeMap, VecDeque};

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
    lights: Vec<i16>,
    // (3) (1,3) (2) (2,3) (0,2) (0,1)
    buttons: Vec<Vec<i16>>,
    // joltage requirements
    joltage: Vec<i16>,
}

impl Machine {
    pub fn new(lights: Vec<i16>, buttons: Vec<Vec<i16>>, joltage: Vec<i16>) -> Self {
        Self {
            lights,
            buttons,
            joltage,
        }
    }

    /// Determine the number of fewest presses to match the indicator lights, e.g. `[.##.]`.
    pub fn light_presses(&self) -> u32 {
        let mut queue: VecDeque<(u32, Vec<i16>)> =
            VecDeque::from([(0, vec![0; self.lights.len()])]);

        loop {
            // get first element
            let (level, lights) = queue.pop_front().expect("Failed to get first element");

            // check if it's the target lights
            if lights == self.lights {
                return level;
            }

            // otherwise press each buttons combination and store to queue.
            for button in self.buttons.iter() {
                let lights = button.iter().fold(lights.clone(), |mut lights, index| {
                    if let Some(entry) = lights.get_mut(*index as usize) {
                        if *entry == 1 {
                            *entry = 0;
                        } else {
                            *entry = 1;
                        }
                    }
                    lights
                });
                queue.push_back((level + 1, lights));
            }
        }
    }

    pub fn joltage_presses(&self) -> u32 {
        let target = vec![0; self.joltage.len()];
        let mut queue: VecDeque<(u32, Vec<i16>)> = VecDeque::from([(0, self.joltage.clone())]);
        let mut cache: BTreeMap<Vec<i16>, u32> = BTreeMap::new();
        let mut num_cache_hits = 0;

        loop {
            let (level, joltage) = queue.pop_front().expect("Failed to find joltage");

            // check if joltage matches target
            if target == joltage {
                println!("RESULT: {}", level);
                return level;
            }

            if let Some(existing_level) = cache.get(&joltage) {
                if *existing_level < level {
                    num_cache_hits += 1;
                    println!(
                        "CACHE HIT: {} - size: {}, level: {}",
                        num_cache_hits,
                        queue.len(),
                        level
                    );
                    continue;
                }
            }
            cache.insert(joltage.clone(), level);

            // if the joltage misses the target, do not continue
            if joltage.iter().all(|item| *item >= 0) {
                // update the target joltage per button
                for button in self.buttons.iter() {
                    let mut new_joltage = joltage.clone();
                    let mut ignore = false;
                    for index in button.iter() {
                        new_joltage[*index as usize] -= 1;
                        if new_joltage[*index as usize] < 0 {
                            ignore = true;
                        }
                    }

                    if !ignore {
                        queue.push_back((level + 1, new_joltage));
                    }
                }
            }
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

fn parse_lights(input: &str) -> IResult<&str, Vec<i16>> {
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

fn parse_buttons_list(input: &str) -> IResult<&str, Vec<Vec<i16>>> {
    separated_list1(space1, parse_buttons).parse(input)
}

fn parse_buttons(input: &str) -> IResult<&str, Vec<i16>> {
    delimited(
        tag("("),
        separated_list1(tag(","), nom::character::complete::i16),
        tag(")"),
    )
    .parse(input)
}

fn parse_joltage(input: &str) -> IResult<&str, Vec<i16>> {
    delimited(
        tag("{"),
        separated_list1(tag(","), nom::character::complete::i16),
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
        assert_eq!(30, machines[0].joltage_presses());
    }
}
