use crate::puzzles::utils;
use std::collections::HashMap;

pub fn run_puzzle_7() {
    let input: String = utils::read_file("input/input_7.txt");

    let mut curr_path: Vec<&str> = vec![];
    let mut dirsizes: HashMap<Vec<&str>, usize> = HashMap::new();

    input.lines().for_each(|l| {
        let bytes = l.as_bytes();
        match bytes[0] {
            b'$' => {
                if bytes[2] == b'c' {
                    match &bytes[5] {
                        b'.' => {
                            curr_path.pop().unwrap();
                        }
                        _ => {
                            curr_path.push(&l[5..]);
                            dirsizes.insert(curr_path.clone(), 0);
                        }
                    }
                }
            }
            b'd' => {} 
            _ => {
                let size = l.split(' ').next().unwrap().parse::<usize>().unwrap();

                for i in (1..curr_path.len() + 1).rev() {
                    *dirsizes.get_mut(&curr_path[0..i]).unwrap() += size;
                }
            }
        }
    });

    // part 1
    let result1 = dirsizes.values().filter(|&&v| v <= 100000).sum::<usize>();

    // part 2
    let root_size = dirsizes.get(&vec!["/"]).unwrap();
    let needed = 70000000 - 30000000;
    let t = root_size - needed;
    let result2 = *dirsizes.values().filter(|&&v| v >= t).min().unwrap();

    println!("------------------------");
    println!("Puzzle 7:");
    println!("Solution 1: {}\nSolution 2: {}", result1, result2);
    println!("------------------------");
}