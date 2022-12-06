use std::{io, env};
use std::collections::VecDeque;
use itertools::Itertools;

fn main() {
    let aoc_phase = env::var("AOC_PHASE").unwrap_or("1".to_string())
        .parse::<usize>().expect("Bad value passed in for AOC_PHASE");

    let buffer_size: usize = match aoc_phase {
        1 => 4,
        2 => 14,
        _ => panic!("Bad AOC_PHASE")
    };

    let mut line = String::new();
    let mut char_buf: VecDeque<char> = VecDeque::<char>::with_capacity(buffer_size);

    while io::stdin().read_line(&mut line).unwrap_or(0) != 0 {
        for (idx, ch) in line.chars().enumerate() {
            if ch == ':' {
                break;
            }
            if char_buf.len() == buffer_size {
                char_buf.pop_front();
            }
            char_buf.push_back(ch);
            //println!("{}: {:?}", idx, char_buf);
            if char_buf.len() != buffer_size {
                continue;
            }
            if char_buf.clone().iter().sorted().dedup().count() == buffer_size {
                println!("Answer: {}", idx+1);
                break;
            }
        }
        char_buf.clear();
        line.clear();
    };
}