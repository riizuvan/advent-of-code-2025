use std::collections::HashMap;

use advent_of_code_2025::fetch_input;

fn euclidean_distance_3d(a: (u64, u64, u64), b: (u64, u64, u64)) -> f64 {
    let dx = (a.0 as i64 - b.0 as i64) as f64;
    let dy = (a.1 as i64 - b.1 as i64) as f64;
    let dz = (a.2 as i64 - b.2 as i64) as f64;

    (dx * dx + dy * dy + dz * dz).sqrt()
}

fn main() -> Result<(), reqwest::Error> {
    let input = fetch_input(8, 2025).expect("failed to fetch input");

    let items = input
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() == 3 {
                Some((
                    parts[0].trim().parse::<u64>().ok()?,
                    parts[1].trim().parse::<u64>().ok()?,
                    parts[2].trim().parse::<u64>().ok()?,
                ))
            } else {
                panic!("Invalid line format: {}", line);
            }
        })
        .collect::<Vec<(u64, u64, u64)>>();

    let mut distances = HashMap::new();

    for (index, item) in items.iter().enumerate() {
        for (other_index, other_item) in items.iter().enumerate() {
            if index != other_index {
                let distance = euclidean_distance_3d(*item, *other_item);

                if distances.contains_key(&(*other_item, *item)) {
                    continue;
                }

                distances.insert((*item, *other_item), distance);
            }
        }
    }

    let mut distances_vec = distances.into_iter().collect::<Vec<_>>();
    distances_vec.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    let mut krugovi = HashMap::new();

    for (index, item) in items.iter().enumerate() {
        krugovi.insert(index, vec![*item]);
    }

    for ((a, b), _distance) in distances_vec.iter() {
        let krug_a = krugovi
            .iter()
            .find(|(_k, v)| v.contains(a))
            .map(|(k, _v)| *k)
            .unwrap();

        let krug_b = krugovi
            .iter()
            .find(|(_k, v)| v.contains(b))
            .map(|(k, _v)| *k)
            .unwrap();

        if krug_a == krug_b {
            continue;
        }

        for item in krugovi.clone().get(&krug_b).unwrap().into_iter() {
            krugovi.get_mut(&krug_a).unwrap().push(*item);
        }

        krugovi.remove(&krug_b);

        if krugovi.len() == 1 {
            println!("{}", a.0 * b.0);

            break;
        }
    }

    Ok(())
}
