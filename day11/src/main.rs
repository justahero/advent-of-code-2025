use std::collections::{HashMap, VecDeque};

/// TODO: for now use String to represent devices.
#[derive(Debug)]
struct Device {
    name: String,
    outputs: Vec<String>,
}

impl Device {
    pub fn new(name: &str, outputs: Vec<&str>) -> Self {
        let outputs = outputs
            .iter()
            .map(|output| output.trim().to_string())
            .collect::<Vec<_>>();

        Self {
            name: name.trim().to_string(),
            outputs,
        }
    }
}

/// Parses a line of the form "aaa: you hhh"
fn parse_device(input: &str) -> Device {
    let (name, outputs) = input.split_once(":").expect("Failed to split on ':'");
    Device::new(name, outputs.split_whitespace().collect::<Vec<_>>())
}

fn parse_input(input: &str) -> Vec<Device> {
    input
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(parse_device)
        .collect::<Vec<_>>()
}

fn find_paths(rack: &HashMap<String, Vec<String>>, start: &str, end: &str) -> u32 {
    let mut total_paths = 0u32;
    let mut queue: VecDeque<_> = VecDeque::from_iter(
        rack.get(start)
            .expect(&format!("Start '{}' not found.", start)),
    );

    while let Some(device) = queue.pop_front() {
        if device == end {
            total_paths += 1;
            continue;
        }

        if device != "out" {
            let outputs = rack
                .get(device)
                .expect(&format!("Failed to find device: {}", device));

            for output in outputs {
                queue.push_back(output);
            }
        }
    }

    total_paths
}

fn process_part1(devices: &[Device]) -> u32 {
    // map all devices to lookup map
    let rack: HashMap<String, Vec<String>> = HashMap::from_iter(
        devices
            .iter()
            .map(|device| (device.name.clone(), device.outputs.clone())),
    );

    find_paths(&rack, "you", "out")
}

/// Start from 'srv' node, collect all paths that pass both 'fft' and 'dac' nodes, there are only two.
fn process_part2(devices: &[Device]) -> u32 {
    // map all devices to lookup map
    let rack: HashMap<String, Vec<String>> = HashMap::from_iter(
        devices
            .iter()
            .map(|device| (device.name.clone(), device.outputs.clone())),
    );

    // first determine the order in which "dac" & "fit" appear
    let a = find_paths(&rack, "svr", "dac")
        * find_paths(&rack, "dac", "fft")
        * find_paths(&rack, "fft", "out");
    let b = find_paths(&rack, "svr", "fft")
        * find_paths(&rack, "fft", "dac")
        * find_paths(&rack, "dac", "out");

    a + b
}

fn main() {
    let devices = parse_input(include_str!("input.txt"));
    let result = process_part1(&devices);
    println!("PART 1: {}", result);
    let result = process_part2(&devices);
    println!("PART 2: {}", result);
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, process_part1, process_part2};

    const INPUT: &str = r#"
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
    "#;

    #[test]
    fn test_parse() {
        let devices = parse_input(INPUT);
        assert_eq!(10, devices.len());
    }

    #[test]
    fn test_part1() {
        let devices = parse_input(INPUT);
        assert_eq!(5, process_part1(&devices));
    }

    #[test]
    fn test_part2() {
        let input: &str = r#"
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
        "#;
        let devices = parse_input(input);
        assert_eq!(2, process_part2(&devices));
    }
}
