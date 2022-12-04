use std::{io, env};

/// # Example
/// ```
/// 2-4,7-6
/// ```
fn main() {
    let aoc_phase = env::var("AOC_PHASE").unwrap_or("1".to_string())
        .parse::<usize>().expect("Bad value passed in for AOC_PHASE");

    let mut line = String::new();
    let mut num_overlaps: u32 = 0;
    while io::stdin().read_line(&mut line).unwrap_or(0) != 0 {
        line = line.trim().to_string();

        // Assumption, get always a pair of ranges

        // Not a huge fan of the intermediate step of storing them as &str's
        // Would much rather go straight from the split into getting u32's
        let ((a_start, a_end), (b_start, b_end)) = parse_pairs(line.clone())
            .expect("Failed to parse a u32 in the line");

        if aoc_phase == 1 {
            // Find the number that overlap completely
            if a_start <= b_start && a_end >= b_end
               || b_start <= a_start && b_end >= a_end {
                num_overlaps += 1;
            }

        } else if aoc_phase == 2 {
            // Find the number that overlap at all
            if a_start <= b_end && b_start <= a_end {
                num_overlaps += 1;
            }
        } else {
            panic!("Unexpected AOC_PHASE")
        }

        line.clear();
    };

    println!("Num overlaps: {}", num_overlaps);
}

fn parse_pairs(line: String) -> Result<((u32, u32), (u32, u32)), std::num::ParseIntError> {
    // Not a huge fan of the intermediate step of storing them as &str's
    // Would much rather go straight from the split into getting u32's
    let (elf_a, elf_b) = line.split_once(',').expect("No ',' found");
    let (a_start_str, a_end_str) = elf_a.split_once('-').expect("No '-' in A");
    let (a_start, a_end) = (a_start_str.parse::<u32>()?, a_end_str.parse::<u32>()?);
    let (b_start_str, b_end_str) = elf_b.split_once('-').expect("No '-' in B");
    let (b_start, b_end) = (b_start_str.parse::<u32>()?, b_end_str.parse::<u32>()?);

    Ok(((a_start, a_end), (b_start, b_end)))
}
