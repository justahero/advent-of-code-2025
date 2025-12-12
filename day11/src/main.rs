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

fn find_paths(rack: &HashMap<&str, Vec<&str>>, start: &str, end: &str) -> u32 {
    let mut total_paths = 0u32;
    let mut queue: VecDeque<&str> = VecDeque::new();
    queue.push_back(start);

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
    let rack: HashMap<&str, Vec<&str>> = HashMap::from_iter(devices.iter().map(|device| {
        (
            device.name.as_str(),
            device
                .outputs
                .iter()
                .map(|output| output.as_str())
                .collect(),
        )
    }));

    find_paths(&rack, "you", "out")
}

fn find_path2(
    devices: &HashMap<String, Vec<String>>,
    from: &str,
    to: &str,
    dac: bool,
    fft: bool,
    cache: &mut HashMap<(String, bool, bool), u64>,
) -> u64 {
    if from == to {
        return if dac && fft { 1 } else { 0 };
    }

    // check if the key has been cached.
    if let Some(count) = cache.get(&(from.to_string(), dac, fft)) {
        return *count;
    }

    let dac = dac || from == "dac";
    let fft = fft || from == "fft";

    if let Some(outputs) = devices.get(from) {
        let sum = outputs
            .iter()
            .map(|from| find_path2(devices, from.as_str(), to, dac, fft, cache))
            .sum();
        cache.insert((from.to_string(), dac, fft), sum);
        sum
    } else {
        0
    }
}

/// Start from 'srv' node, collect all paths that pass both 'fft' and 'dac' nodes, there are only two.
fn process_part2(devices: &[Device]) -> u64 {
    // map all devices to lookup map, for now use String
    let devices: HashMap<String, Vec<String>> = HashMap::from_iter(
        devices
            .iter()
            .map(|device| (device.name.clone(), device.outputs.clone())),
    );

    find_path2(&devices, "svr", "out", false, false, &mut HashMap::new())
}

fn main() {
    let devices = parse_input(include_str!("input.txt"));
    let result = process_part1(&devices);
    println!("PART 1: {}", result);
    let result = process_part2(&devices);
    println!("PART 2: {}", result);
    // 2100348264 too low
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
