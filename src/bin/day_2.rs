use advent_of_code_2025::fetch_input;
use fancy_regex::Regex;

fn main() -> Result<(), reqwest::Error> {
    let input = fetch_input(2, 2025).expect("failed to fetch input");

    let characters = input
        .lines()
        .next()
        .expect("input is empty")
        .split(',')
        .map(|range| {
            let (start, end) = range.split_at(range.find('-').expect("invalid range"));
            let start: u64 = start.parse().expect("failed to parse start");
            let end: u64 = end[1..].parse().expect("failed to parse end");

            (start, end)
        });

    let invalid_id_match = Regex::new(r"^(\d+)\1$").unwrap();

    let mut sum = 0;

    for range in characters {
        for id in range.0..=range.1 {
            let id_str = id.to_string();

            if invalid_id_match.is_match(&id_str).unwrap() {
                sum += id;
            }
        }
    }

    println!("{}", sum);

    Ok(())
}
