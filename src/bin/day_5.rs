use advent_of_code_2025::fetch_input;

fn main() -> Result<(), reqwest::Error> {
    let input = fetch_input(5, 2025).expect("failed to fetch input");

    let [ranges_input, ids_input] = input
        .split("\n\n")
        .collect::<Vec<&str>>()
        .try_into()
        .unwrap();

    let ranges = ranges_input
        .lines()
        .map(|line| {
            let [start, end] = line.split('-').collect::<Vec<&str>>().try_into().unwrap();

            (start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap())
        })
        .collect::<Vec<(u64, u64)>>();

    let ids = ids_input
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let mut fresh_ingredients_count = 0;

    for id in ids {
        let is_fresh = ranges.iter().any(|(start, end)| id >= *start && id <= *end);

        if is_fresh {
            fresh_ingredients_count += 1;
        }
    }

    println!("{}", fresh_ingredients_count);

    Ok(())
}
