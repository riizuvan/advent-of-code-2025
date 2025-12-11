use std::collections::HashMap;

use advent_of_code_2025::fetch_input;

fn count_paths(
    device: &str,
    devices_map: &HashMap<&str, Vec<&str>>,
    visited: &mut HashMap<String, u64>,
) -> u64 {
    if device == "out" {
        return 1;
    }

    if let Some(&count) = visited.get(device) {
        return count;
    }

    let mut total_paths = 0;

    if let Some(neighbors) = devices_map.get(device) {
        for &neighbor in neighbors {
            total_paths += count_paths(neighbor, devices_map, visited);
        }
    }

    visited.insert(device.to_string(), total_paths);

    total_paths
}

fn main() -> Result<(), reqwest::Error> {
    let input = fetch_input(11, 2025).expect("failed to fetch input");

    let devices = input
        .lines()
        .map(|line| {
            let parts = line.split(" ").collect::<Vec<&str>>();
            let device = parts.get(0).unwrap().trim_matches(|c| c == ':');

            (device, parts[1..].to_vec())
        })
        .collect::<Vec<(&str, Vec<&str>)>>();

    let mut devices_map = HashMap::new();

    for (device, outs) in devices {
        devices_map.insert(device, outs);
    }

    let result = count_paths("you", &devices_map, &mut HashMap::new());

    println!("{}", result);

    Ok(())
}
