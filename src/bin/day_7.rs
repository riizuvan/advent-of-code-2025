use std::collections::HashMap;

use advent_of_code_2025::fetch_input;

fn beam_search(
    position: (i32, i32),
    matrix: &Vec<Vec<char>>,
    split_count: &mut i32,
    visited: &mut HashMap<(i32, i32), bool>,
) {
    let item = matrix
        .get(position.1 as usize)
        .and_then(|row| row.get(position.0 as usize));

    if item.is_none() {
        return;
    }

    if visited.contains_key(&position) {
        return;
    }

    visited.insert(position, true);

    if matrix[position.1 as usize][position.0 as usize] != '^' {
        beam_search((position.0, position.1 + 1), matrix, split_count, visited);
    } else {
        *split_count += 1;

        beam_search((position.0 + -1, position.1), matrix, split_count, visited);
        beam_search((position.0 + 1, position.1), matrix, split_count, visited);
    }
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

    let mut start: (i32, i32) = (0, 0);

    for y in 0..matrix.len() {
        for x in 0..matrix[y].len() {
            let item = matrix[y][x];

            if item == 'S' {
                start = (x as i32, y as i32);
            }
        }
    }

    let mut split_count = 0;

    let mut visited = HashMap::new();

    beam_search(start, &matrix, &mut split_count, &mut visited);

    println!("{}", split_count);

    Ok(())
}
