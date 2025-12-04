use advent_of_code_2025::fetch_input;

fn main() -> Result<(), reqwest::Error> {
    let input = fetch_input(4, 2025).expect("failed to fetch input");

    let mut matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let rolls_count = matrix
        .iter()
        .map(|row| row.iter().filter(|&&ch| ch == '@').count())
        .sum::<usize>();

    loop {
        let mut accesable_rolls = vec![];

        for (y, row) in matrix.iter().enumerate() {
            'chars_loop: for (x, &ch) in row.iter().enumerate() {
                if ch != '@' {
                    continue;
                }

                let adjecent_eight = [
                    (-1, -1),
                    (0, -1),
                    (1, -1),
                    (-1, 0),
                    (1, 0),
                    (-1, 1),
                    (0, 1),
                    (1, 1),
                ];

                let mut blocked_sides = 0;

                for (dx, dy) in adjecent_eight.iter() {
                    let adjecent_char = matrix
                        .get((y as isize + *dy) as usize)
                        .and_then(|r| r.get((x as isize + *dx) as usize))
                        .unwrap_or(&'.');

                    if *adjecent_char == '@' {
                        blocked_sides += 1;
                    }

                    if blocked_sides > 3 {
                        continue 'chars_loop;
                    }
                }

                accesable_rolls.push((x, y));
            }
        }

        if accesable_rolls.len() == 0 {
            let new_rolls_count = matrix
                .iter()
                .map(|row| row.iter().filter(|&&ch| ch == '@').count())
                .sum::<usize>();

            println!("{}", rolls_count - new_rolls_count);

            break;
        }

        let (x, y) = accesable_rolls.pop().unwrap();

        matrix[y][x] = '.';
    }

    Ok(())
}
