use std::io;

fn main() {
    let mut line = String::new();

    let mut calories_by_elf: Vec<u32> = Vec::<u32>::new();
    let mut current_total: u32 = 0;
    while io::stdin().read_line(&mut line).unwrap_or(0) != 0 {
        line = line.trim().to_string();
        if line.len() == 0 {
            calories_by_elf.push(current_total);

            current_total = 0;
        } else {
            current_total += line.parse::<u32>().unwrap();
        }

        line.clear();
    }
    calories_by_elf.sort_unstable_by(|a, b| b.cmp(a));
    println!("Answer: {:?}", calories_by_elf[0..3].iter().sum::<u32>());
}
