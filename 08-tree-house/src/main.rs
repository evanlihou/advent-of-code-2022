/// I'll be honest, I'm not super proud of this one.
use std::{io, env};

#[derive(Debug)]
pub struct Tree {
    height: u8,
    // Used in part 1
    visible: bool
}

impl Tree {
    pub fn set_visible(&mut self) {
        self.visible = true;
    }
}

fn main() {
    let aoc_phase = env::var("AOC_PHASE").unwrap_or("1".to_string())
        .parse::<usize>().expect("Bad value passed in for AOC_PHASE");

    let mut line = String::new();

    let mut tree_vec: Vec<Vec<Tree>> = Vec::<Vec<Tree>>::new();

    while io::stdin().read_line(&mut line).unwrap_or(0) != 0 {
        tree_vec.push(line.trim().chars().map(|c|
        Tree {
            height: c.to_digit(10).expect("Hit non-digit") as u8,
            visible: false
        }
        ).collect());

        line.clear();
    };

    if aoc_phase == 1 {
        for row in tree_vec.iter_mut() {
            calc_visible(row);
        }
        // It feels pretty wasteful to transpose the whole structure, but I
        // couldn't get it to work any other way due to Rust being confusing
        let mut transposed_tree_vec = transpose(tree_vec);
        for row in transposed_tree_vec.iter_mut() {
            calc_visible(row);
        }
        println!("{:?}", transposed_tree_vec.into_iter().flatten().filter(|c| c.visible == true).count());
    } else if aoc_phase == 2 {
        let mut best_score: u32 = 0;
        let height = tree_vec.len();
        let width = tree_vec.get(0).expect("Empty tree_vec?").len();
        for y in 0..height {
            for x in 0..width {
                let mut tree_score: u32 = 0;
                let blocking_height: u8 = tree_vec[y][x].height;
                let mut num_trees_visible: u32 = 0;
                // Up
                for search_y in (0..y).rev() {
                    let tree = tree_vec[search_y][x].height;
                    num_trees_visible += 1;
                    if tree >= blocking_height {
                        break;
                    }
                }
                tree_score += num_trees_visible;
                num_trees_visible = 0;
                // Down
                for search_y in y+1..height {
                    let tree = tree_vec[search_y][x].height;
                    num_trees_visible += 1;
                    if tree >= blocking_height {
                        break;
                    }
                }
                tree_score *= num_trees_visible;
                num_trees_visible = 0;
                // Left
                for search_x in (0..x).rev() {
                    let tree = tree_vec[y][search_x].height;
                    num_trees_visible += 1;
                    if tree >= blocking_height {
                        break;
                    }
                }
                tree_score *= num_trees_visible;
                num_trees_visible = 0;
                // Right
                for search_x in x+1..width {
                    let tree = tree_vec[y][search_x].height;
                    num_trees_visible += 1;
                    if tree >= blocking_height {
                        break;
                    }
                }
                tree_score *= num_trees_visible;

                if tree_score > best_score {
                    best_score = tree_score;
                }

            }
        }

        println!("Best score: {}", best_score);
    } else {
        panic!("Unexpected AOC_PHASE")
    }
}

fn calc_visible(trees: &mut [Tree]) -> u32
{
    //let mut a = trees.clone();
    let mut visible_count: u32 = 0;
    let mut max: u8 = 0;
    let mut first_tree = true;
    for tree in trees.iter_mut() {
        if first_tree || max < tree.height {
            max = tree.height;
            tree.set_visible();
            visible_count += 1;
        }
        first_tree = false;
    }

    max = 0;
    first_tree = true;
    for tree in trees.iter_mut().rev() {
        if first_tree || max < tree.height {
            max = tree.height;
            tree.set_visible();
            visible_count += 1;
        }
        first_tree = false;
    }

    visible_count
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
    .map(|_| {
        iters
        .iter_mut()
        .map(|n| n.next().unwrap())
        .collect::<Vec<T>>()
    })
    .collect()
}