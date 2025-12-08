use std::collections::HashMap;

use advent_of_code_2025::fetch_input;

fn beam_search(
    position: (i64, i64),
    matrix: &Vec<Vec<char>>,
    cache: &mut HashMap<(i64, i64), u64>,
) -> u64 {
    if cache.contains_key(&position) {
        return *cache.get(&position).unwrap();
    }

    // overflow down couns as one step after split
    if position.1 == matrix.len() as i64 {
        cache.insert(position, 1);

        return 1;
    }

    // overflow left or right means no space to split
    if position.0 >= matrix[0].len() as i64 || position.0 < 0 {
        cache.insert(position, 0);

        return 0;
    }

    let mut timelines = 0;

    if matrix[position.1 as usize][position.0 as usize] != '^' {
        timelines = beam_search((position.0, position.1 + 1), matrix, cache);
    } else {
        timelines += beam_search((position.0 + -1, position.1), matrix, cache);
        timelines += beam_search((position.0 + 1, position.1), matrix, cache);
    }

    cache.insert(position, timelines);

    timelines
}

fn main() -> Result<(), reqwest::Error> {
    let input = fetch_input(7, 2025).expect("failed to fetch input");

    let matrix = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut nodes = HashMap::new();

    for y in 0..matrix.len() {
        for x in 0..matrix[y].len() {
            let item = matrix[y][x];

            nodes.insert((x, y), item);
        }
    }

    let mut start: (i64, i64) = (0, 0);

    for y in 0..matrix.len() {
        for x in 0..matrix[y].len() {
            let item = matrix[y][x];

            if item == 'S' {
                start = (x as i64, y as i64);
            }
        }
    }

    let mut cache = HashMap::new();

    let timelines = beam_search(start, &matrix, &mut cache);

    println!("{}", timelines);

    Ok(())
}
