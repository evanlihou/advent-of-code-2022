use std::{io, env};
use std::collections::VecDeque;
use regex::Regex;
use lazy_static::lazy_static;

fn main() {
    let aoc_phase = env::var("AOC_PHASE").unwrap_or("1".to_string())
        .parse::<usize>().expect("Bad value passed in for AOC_PHASE");

    let mut line = String::new();

    let mut stacks: Vec<VecDeque<char>> = Vec::<VecDeque<char>>::new();
    let mut is_initializing = true;

    while io::stdin().read_line(&mut line).unwrap_or(0) != 0 {
        // When we encounter an empty line, switch from filling the stacks to
        // moving them
        if line.trim().len() == 0 {
            is_initializing = false;
            line.clear();
            continue;
        }

        if is_initializing {
            // Fill up the stacks. Whitespace matters in this bit, it tells us
            // which index we need to put something in.
            for (idx, ch) in line.chars().enumerate() {
                if ch.is_numeric() {
                    break;
                }

                if ch.is_alphabetic() {
                    let stack_num = idx / 4 as usize;
                    if stacks.len() < stack_num+1 {
                        for _ in 0..stack_num-stacks.len()+1 {
                            stacks.push(VecDeque::new())
                        }
                    }
                    stacks[stack_num].push_front(ch);
                }
            }
        } else {
            // Move things around on the stacks
            line = line.trim().to_string();
            let (iters, from, to) = parse_move_line(line.as_str());
            if aoc_phase == 1 {
                // Pick up items one at a time
                for _ in 0..iters {
                    let item = stacks[from].pop_back()
                        .expect("Hit an empty stack!!");
                    stacks[to].push_back(item);
                }
            } else if aoc_phase == 2 {
                // Pick up multiple items at the same time, preserving the order
                // when putting them back down
                let drain_start = stacks[from].len()-iters;
                let items = stacks[from].drain(drain_start..)
                    .collect::<VecDeque<char>>();
                stacks[to].extend(items);
            } else {
                panic!("Unexpected AOC_PHASE")
            }
        }

        line.clear();
    };
    println!("{:?}", stacks.iter().map(|s| s.iter().last().expect("Empty stack"))
        .collect::<String>())
}

fn parse_move_line(line: &str) -> (usize, usize, usize) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^move (\d+) from (\d+) to (\d+)$")
            .unwrap();
    }
    let matches = RE.captures(line).expect("Hit a malformed line");

    (matches[1].parse::<usize>().expect("Bad iters"),
     matches[2].parse::<usize>().expect("Bad from") - 1,
     matches[3].parse::<usize>().expect("Bad to") - 1)
}
