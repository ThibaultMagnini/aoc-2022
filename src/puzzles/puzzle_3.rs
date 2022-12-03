use crate::puzzles::utils;
use std::collections::HashSet;

static ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"; 

pub fn run_puzzle_3() {
    let input: String = utils::read_file("input/input_3.txt");

    let mut total_prio: u32 = 0;
    let mut group_total_prio: u32 = 0; 
    let mut group_of_3: Vec<HashSet<char>> = Vec::new();

    for line in input.lines() {
        // Puzzle 1
        let (compartment1, compartment2) = line.split_at(line.len() / 2);
        total_prio += get_priority_1(compartment1, compartment2);
        
        // Puzzle 2
        group_of_3.push(line.chars().collect());
        
        if group_of_3.len() == 3 {
            group_total_prio += get_priority_2(&group_of_3);
            group_of_3.clear();
        }
    }
    
    println!("Puzzle 3:");
    println!("Total Prio: {},\nTotal Prio By 3: {}", total_prio, group_total_prio);
    println!("------------------------");
}


fn get_priority_1(compartment1: &str, compartment2: &str) -> u32{
    let mut prio: u32 = 0;

    for el1 in compartment1.chars() {
        for el2 in compartment2.chars() {
            if el1.eq(&el2) {
                prio += ALPHABET.find(el1).unwrap() as u32 + 1;
                return prio
            }
        }
    }
    0
}


fn get_priority_2(group_of_3: &Vec<HashSet<char>>) -> u32 {
    // There probably is a better way to determine the intersection of 3 sets...
    let mut iter = group_of_3.iter();
    let base: HashSet<char> = iter.next().unwrap().clone();
    let intersection: HashSet<char> = iter.fold(base, |acc, set| acc.intersection(set).map(|x| x.clone()).collect());
    ALPHABET.find(*intersection.iter().next().unwrap()).unwrap() as u32 + 1
}