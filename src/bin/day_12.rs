use advent_of_code_2025::fetch_input;

fn main() -> Result<(), reqwest::Error> {
    let input = fetch_input(12, 2025).expect("failed to fetch input");

    let regions = input
        .lines()
        .filter(|line| line.contains("x"))
        .map(|line| {
            let parts = line.split(" ").collect::<Vec<&str>>();

            let size = parts[0]
                .trim_matches(|c| c == ':')
                .split('x')
                .map(|s| s.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();

            let presents = parts[1..]
                .iter()
                .map(|&s| s.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();

            (size, presents)
        })
        .collect::<Vec<(Vec<u64>, Vec<u64>)>>();

    let mut valid = 0;

    for (size, presents) in regions {
        let area: u64 = size.iter().product();

        let total_presents: u64 = presents.iter().map(|item| item * 9).sum();

        if total_presents <= area {
            valid += 1;
        }
    }

    println!("{}", valid);

    Ok(())
}
