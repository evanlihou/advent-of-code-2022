use std::{io, collections};

// How big should the sliding window be?
const WINDOW_SIZE: usize = 3;

fn main() {
    let mut line = String::new();

    let mut last_reading = std::u32::MAX;
    let mut sliding_window_readings = collections::VecDeque::<u32>::with_capacity(WINDOW_SIZE);
    let mut num_increases: u32 = 0;

    while io::stdin().read_line(&mut line).unwrap_or(0) != 0 {
        if sliding_window_readings.len() == WINDOW_SIZE {
            sliding_window_readings.pop_back();
        }

        sliding_window_readings.push_front(line.trim().parse::<u32>().unwrap());
        line.clear();

        if sliding_window_readings.len() != WINDOW_SIZE {
            // Only start doing calculations once we have a full window
            continue
        }

        //println!("{:?}", sliding_window_readings);

        let reading: u32 = sliding_window_readings.iter().sum::<u32>();
        if reading > last_reading {
            num_increases += 1;
        }

        last_reading = reading;
    };

    println!("Increased {} times!", num_increases)
}
