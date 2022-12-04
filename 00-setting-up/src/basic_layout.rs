use std::{io, env};

fn main() {
    let aoc_phase = env::var("AOC_PHASE").unwrap_or("1".to_string())
        .parse::<usize>().expect("Bad value passed in for AOC_PHASE");

    let mut line = String::new();

    while io::stdin().read_line(&mut line).unwrap_or(0) != 0 {
        println!("{}", line);
        if aoc_phase == 1 {
            println!("Doing phase 1");
        } else if aoc_phase == 2 {
            println!("Doing phase 2");
        } else {
            panic!("Unexpected AOC_PHASE")
        }

        line.clear();
    };
}
