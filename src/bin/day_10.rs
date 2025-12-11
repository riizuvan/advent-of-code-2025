use advent_of_code_2025::fetch_input;
use z3::{ast::Int, Optimize, SatResult};

#[derive(Debug)]
struct Machine {
    target: Vec<usize>,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<usize>,
}

fn solve_machine(machine: &Machine) -> u64 {
    // Use the crate's implicit thread-local context and an Optimize instance
    let opt = Optimize::new();

    let n_buttons = machine.buttons.len();
    let n_counters = machine.joltage.len();

    // press[b] is an Int (number of times button b is pressed)
    let press: Vec<Int> = (0..n_buttons)
        .map(|i| Int::new_const(format!("press_{}", i)))
        .collect();

    // Constrain presses to be non-negative integers
    let zero = Int::from_u64(0);
    for p in &press {
        opt.assert(&p.ge(&zero));
    }

    // For each counter, the sum of presses of buttons that affect it must equal the target joltage
    for i in 0..n_counters {
        let mut terms: Vec<Int> = Vec::new();
        for (b, btn) in machine.buttons.iter().enumerate() {
            if btn.contains(&i) {
                terms.push(press[b].clone());
            }
        }

        let sum_i = if terms.is_empty() {
            Int::from_u64(0)
        } else {
            terms.into_iter().sum::<Int>()
        };

        let target_i = Int::from_u64(machine.joltage[i] as u64);
        opt.assert(&sum_i.eq(&target_i));
    }

    // Minimize total presses
    let sum = Int::new_const("press_sum");
    let total = if press.is_empty() {
        Int::from_u64(0)
    } else {
        press.into_iter().sum::<Int>()
    };
    opt.assert(&sum.eq(&total));
    opt.minimize(&sum);

    // Solve
    let result = opt.check(&[]);
    if result != SatResult::Sat {
        panic!("No SAT solution found");
    }

    let model = opt.get_model().unwrap();
    model
        .get_const_interp(&sum)
        .expect("Missing model interpretation")
        .as_u64()
        .expect("Expected unsigned integer")
}

fn main() -> Result<(), reqwest::Error> {
    let input = fetch_input(10, 2025).expect("failed to fetch input");

    let matchines = input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(' ').collect();
            let target: Vec<usize> = parts[0]
                .trim_matches(|c| c == '[' || c == ']')
                .chars()
                .map(|s| if s == '.' { 0 } else { 1 })
                .collect();

            let buttons: Vec<Vec<usize>> = parts[1..parts.len() - 1]
                .iter()
                .map(|part| {
                    part.trim_matches(|c| c == '(' || c == ')')
                        .split(',')
                        .map(|s| s.parse::<usize>().unwrap())
                        .collect()
                })
                .collect();
            let joltage: Vec<usize> = parts[parts.len() - 1]
                .trim_matches(|c| c == '{' || c == '}')
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect();

            Machine {
                target,
                buttons,
                joltage,
            }
        })
        .collect::<Vec<Machine>>();

    let results = matchines
        .iter()
        .map(|machine| solve_machine(machine))
        .collect::<Vec<u64>>();

    println!("{:?}", results.iter().sum::<u64>());

    Ok(())
}
