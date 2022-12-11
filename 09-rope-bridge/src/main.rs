use std::{io, env};
use std::collections::{HashSet};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Position {
    x: i32,
    y: i32
}

impl Position {
    pub fn get_diagonal<'a>(self) -> Vec<Position> {
        vec![
            Position { x: self.x.saturating_sub(1), y: self.y.saturating_sub(1) },
            Position { x: self.x.saturating_add(1), y: self.y.saturating_sub(1) },
            Position { x: self.x.saturating_sub(1), y: self.y.saturating_add(1) },
            Position { x: self.x.saturating_add(1), y: self.y.saturating_add(1) },
        ]
    }

    pub fn get_orthagonal<'a>(self) -> Vec<Position> {
        vec![
        Position { x: self.x, y: self.y.saturating_add(1) },
        Position { x: self.x, y: self.y.saturating_sub(1) },
        Position { x: self.x.saturating_sub(1), y: self.y },
        Position { x: self.x.saturating_add(1), y: self.y },
        ]
    }

    pub fn is_adjacent(self, b: &Position) -> bool {
        return self.x.abs_diff(b.x) <= 1 && self.y.abs_diff(b.y) <= 1;
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    pub fn from(c: char) -> Self {
        match c {
            'U' => Self::Up,
            'D' => Self::Down,
            'L' => Self::Left,
            'R' => Self::Right,
            _ => panic!("Unexpected Direction {}", c)
        }
    }

    pub fn new_pos(&self, pos: &Position) -> Position {
        match self {
            Direction::Up    => Position { x: pos.x, y: pos.y.saturating_add(1)},
            Direction::Down  => Position { x: pos.x, y: pos.y.saturating_sub(1)},
            Direction::Left  => Position { x: pos.x.saturating_sub(1), y: pos.y},
            Direction::Right => Position { x: pos.x.saturating_add(1), y: pos.y}
        }
    }
}

fn main() {
    let aoc_phase = env::var("AOC_PHASE").unwrap_or("1".to_string())
    .parse::<usize>().expect("Bad value passed in for AOC_PHASE");

    let mut line = String::new();

    let mut rope: Vec<Position> = vec![Position {x: 0, y: 0}; if aoc_phase == 1 { 2 } else { 10 }];
    let mut visited_locations: HashSet<Position> = HashSet::<Position>::new();
    visited_locations.insert(Position { x: 0, y: 0 });

    while io::stdin().read_line(&mut line).unwrap_or(0) != 0 {
        let direction = Direction::from(line.chars().nth(0).unwrap());
        let count = line[2..].trim().parse::<u32>().unwrap();
        simulate_rope(&mut rope, &mut visited_locations, count, direction);

        line.clear();
    };

        for line in (-20..20).rev() {
            println!("{}", (-20..20).map(|i| if visited_locations.contains(&Position {x: i, y: line}) { '#' } else { ' ' }).collect::<String>());
        }
    println!("Num visited: {}", visited_locations.len());
}

fn simulate_rope<'a>(rope: &mut [Position], visited_locations: &mut HashSet<Position>, count: u32, direction: Direction) {
    for _ in 0..count {
        rope[0] = direction.new_pos(&rope[0]);
        for i in 0..rope.len()-1 {
            let head = rope[i];
            let tail = rope[i+1];
            let is_last = rope.len() - 2 == i;
            if !head.is_adjacent(&tail) {
                if head.x == tail.x || head.y == tail.y {
                    let mut found = false;
                    for possible in tail.get_orthagonal() {
                        if head.is_adjacent(&possible) {
                            rope[i+1] = possible;
                            if is_last {
                                visited_locations.insert(possible);
                            }
                            found = true;
                            break;
                        }
                    }
                    if !found || !head.is_adjacent(&rope[i+1]) {
                        panic!("Couldn't figure out where to move tail");
                    }
                    continue;
                }
                let mut found = false;
                for possible in tail.get_diagonal() {
                    if head.is_adjacent(&possible) {
                        rope[i+1] = possible;
                        if is_last {
                            visited_locations.insert(possible);
                        }
                        found = true;
                        break;
                    }
                }
                if !found || !head.is_adjacent(&rope[i+1]) {
                    panic!("Couldn't figure out where to move tail");
                }
            }
        }
        //println!("Rope: {:?}", rope);
    }

//    for line in (-20..20).rev() {
//        println!("{}", (-20..20).map(|i| if rope.contains(&Position {x: i, y: line}) { '#' } else { '.' }).collect::<String>());
//    }
//    println!();
}