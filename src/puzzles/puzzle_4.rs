use crate::puzzles::utils;
use std::collections::HashSet;

pub fn run_puzzle_4() {
    let input: String = utils::read_file("input/input_4.txt");

    let mut count = 0;
    let mut count_part2: u32 = 0;

    for line in input.lines() {
        let parts: Vec<&str> = line.split(",").collect();
        
        let temp: Vec<&str> = parts[0].split("-").collect();
        let temp2: Vec<&str> = parts[1].split("-").collect();
        
        let part1start_int = temp[0].parse::<u32>().unwrap();
        let part2start_int = temp2[0].parse::<u32>().unwrap();
        let part1stop_int = temp[1].parse::<u32>().unwrap();
        let part2stop_int = temp2[1].parse::<u32>().unwrap();

        count += part_1(&part1start_int, &part2start_int, &part1stop_int, &part2stop_int);
        count_part2 += part_2(&part1start_int, &part2start_int, &part1stop_int, &part2stop_int);

    }
    println!("Puzzle 4:");
    println!("Total Count: {},\nTotal Count part 2: {}", count, count_part2);
    println!("------------------------");
}


fn part_1(part1start_int: &u32, part2start_int: &u32, part1stop_int: &u32, part2stop_int: &u32) -> u32 {
    if (part1start_int <= part2start_int) & (part1stop_int >= part2stop_int) {
        return 1
    }
    else if (part1start_int >= part2start_int) & (part1stop_int <= part2stop_int) {
        return 1
    }
    return 0
}

fn part_2(part1start_int: &u32, part2start_int: &u32, part1stop_int: &u32, part2stop_int: &u32) -> u32 {
    let set1: HashSet<u32> = (*part1start_int..*part1stop_int + 1).collect();
    let set2: HashSet<u32> = (*part2start_int..*part2stop_int + 1).collect();

    let inter = set1.intersection(&set2);

    if inter.count() > 0 {
       return 1
    } 
    return 0
}