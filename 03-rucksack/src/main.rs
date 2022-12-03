use std::{io, env};
use std::collections::HashSet;

const GROUP_SIZE: usize = 3;

/// Get the priority for a given character
fn get_priority(ch: &char) -> u32 {
    return match ch {
        'a'..='z' => *ch as u32 - 'a' as u32,
        'A'..='Z' => *ch as u32 - 'A' as u32 + 26,
        _ => panic!("get_priority only supports [A-Za-z]")
    } + 1;
}

fn main() {
    let aoc_phase = env::var("AOC_PHASE").unwrap_or("1".to_string())
        .parse::<usize>().expect("Bad value passed in for AOC_PHASE");

    let mut priority_sum = 0;

    // Phase 2: Create groups of `x` lines
    let mut group = Vec::<HashSet<char>>::with_capacity(GROUP_SIZE);

    let mut line = String::new();
    while io::stdin().read_line(&mut line).unwrap_or(0) != 0 {
        if aoc_phase == 1 {
            // Find the character that's both in the first and last half of the line
            let (begin, end) = line.split_at(line.len() / 2);
            let begin_hashmap = begin.chars().collect::<HashSet<char>>();
            let end_hashmap = end.chars().collect::<HashSet<char>>();
            let duplicate = begin_hashmap.intersection(&end_hashmap).nth(0)
                .expect("Couldn't find a duplicate item");
            let priority: u32 = get_priority(duplicate);
            priority_sum += priority;

        } else if aoc_phase == 2 {
            // Find the character that's shared between all lines in `group`
            line = line.trim().to_string();
            group.push(line.chars().collect::<HashSet<char>>());
            if group.len() == GROUP_SIZE {
                let duplicate = group.iter().fold(group[0].clone(), |acc, hs| {
                    acc.intersection(hs).cloned().collect()
                });
                let priority = get_priority(duplicate.iter().nth(0)
                    .expect("Didn't get a duplicate"));
                priority_sum += priority;
                group.clear();
            }
        } else {
            panic!("Unexpected AOC_PHASE")
        }
        line.clear();
    };
    println!("Result: {}", priority_sum);
}
