use crate::puzzles::utils;
use std::collections::HashSet;

pub fn run_puzzle_6() {
    let input: String = utils::read_file("input/input_6.txt");

    let mut index_1: usize = 0;
    let mut index_2: usize = 0;

    let chars = input.lines().collect::<Vec<&str>>()[0]
        .chars()
        .collect::<Vec<char>>();

    for (i, window) in chars.windows(4).enumerate() {
        if window.iter().collect::<HashSet<_>>().len() == 4 {
            index_1 = i + 4;
            break;
        }
    }

    for (i, window) in chars.windows(14).enumerate() {
        if window.iter().collect::<HashSet<_>>().len() == 14 {
            index_2 = i + 14;
            break;
        }
    }

    println!("------------------------");
    println!("Puzzle 6:");
    println!("Solution 1: {:?}\nSolution 2: {:?}", index_1, index_2);
    println!("------------------------");
}