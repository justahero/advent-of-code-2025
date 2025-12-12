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

fn process_part1(devices: &[Device]) -> u32 {
    // map all devices to lookup map
    let rack: HashMap<String, Vec<String>> = HashMap::from_iter(
        devices
            .iter()
            .map(|device| (device.name.clone(), device.outputs.clone())),
    );

    let mut total_paths = 0u32;
    let mut queue: VecDeque<_> = VecDeque::from_iter(rack.get("you").expect("No 'you' device"));

    while let Some(device) = queue.pop_front() {
        let outputs = rack.get(device).expect("Failed to find device");
        for output in outputs {
            if output == "out" {
                total_paths += 1;
            } else {
                queue.push_back(output);
            }
        }
    }

    total_paths
}

fn main() {
    let devices = parse_input(include_str!("input.txt"));
    let result = process_part1(&devices);
    println!("PART 1: {}", result);
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, process_part1};

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
}
