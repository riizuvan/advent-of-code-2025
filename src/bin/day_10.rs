use advent_of_code_2025::fetch_input;
use z3::{
    ast::{Bool, Int},
    Optimize, SatResult,
};

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
    let n_lights = machine.target.len();

    // press[b] is a Bool (pressed or not)
    let press: Vec<Bool> = (0..n_buttons)
        .map(|i| Bool::new_const(format!("press_{}", i)))
        .collect();

    // For each light, construct XOR of all press[b] that toggle it
    for light in 0..n_lights {
        let mut togglers: Vec<Bool> = Vec::new();

        for (b, btn) in machine.buttons.iter().enumerate() {
            if btn.contains(&light) {
                togglers.push(press[b].clone());
            }
        }

        // XOR chain using Bool::xor
        let xor_expr = if togglers.is_empty() {
            Bool::from_bool(false)
        } else if togglers.len() == 1 {
            togglers[0].clone()
        } else {
            let mut it = togglers.into_iter();
            let first = it.next().unwrap();
            it.fold(first, |acc, b| acc.xor(&b))
        };

        // Compare XOR result to target bit
        let target_bool = Bool::from_bool(machine.target[light] == 1);
        opt.assert(&xor_expr.eq(&target_bool));
    }

    // Minimize sum of pressed buttons
    let sum = Int::new_const("press_sum");

    let mut terms: Vec<Int> = Vec::new();
    for p in &press {
        let one = Int::from_u64(1);
        let zero = Int::from_u64(0);
        terms.push(p.ite(&one, &zero));
    }

    let total = terms.into_iter().sum::<Int>();
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
