use std::collections::HashMap;

use advent_of_code_2025::fetch_input;

fn count_paths(
    device: &str,
    devices_map: &HashMap<&str, Vec<&str>>,
    visited: &mut HashMap<String, u64>,
    out: &str,
) -> u64 {
    if device == out {
        return 1;
    }

    if let Some(&count) = visited.get(device) {
        return count;
    }

    let mut total_paths = 0;

    if let Some(neighbors) = devices_map.get(device) {
        for &neighbor in neighbors {
            total_paths += count_paths(neighbor, devices_map, visited, out);
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

    let svr_fft = count_paths("svr", &devices_map, &mut HashMap::new(), "fft");
    let fft_dac = count_paths("fft", &devices_map, &mut HashMap::new(), "dac");
    let dac_out = count_paths("dac", &devices_map, &mut HashMap::new(), "out");

    let result = svr_fft * fft_dac * dac_out;

    println!("{}", result);

    Ok(())
}
