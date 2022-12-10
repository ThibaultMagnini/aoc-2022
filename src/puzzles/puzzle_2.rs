use crate::puzzles::utils;
use std::collections::HashMap;

pub fn run_puzzle_2() {
    let input: String = utils::read_file("input/input_2.txt");

    let mut total_score_1: i32 = 0;
    let mut total_score_2: i32 = 0;

    for line in input.lines() {
        let actions: Vec<&str> = line.split(' ').collect();
        total_score_1 += calculate_score_first(&actions);
        total_score_2 += calculate_score_second(&actions);
    }

    println!("------------------------");
    println!("Puzzle 2:");
    println!("Total score 1: {0},\nTotal score 2: {1}", total_score_1, total_score_2);
    println!("------------------------");
}

fn calculate_score_first(actions: &Vec<&str>) -> i32{
    let draw = HashMap::from([("X", "A"),("Y", "B"),("Z", "C"),]);
    let points = HashMap::from([("X", 1),("Y", 2),("Z", 3),]);
    let victory = HashMap::from([("X", "C"),("Y", "A"),("Z", "B"),]);

    let mut count: i32 = 0;

    if actions[0].eq(*draw.get(actions[1]).unwrap()) {
        count += points.get(actions[1]).unwrap() + 3;
    } else if actions[0].eq(*victory.get(actions[1]).unwrap()) {
        count += points.get(actions[1]).unwrap() + 6;
    } else {
        count += points.get(actions[1]).unwrap();
    }
    count
}


fn calculate_score_second(actions: &Vec<&str>) -> i32{
    let to_pick_draw = HashMap::from([("A", "X"),("B", "Y"),("C", "Z"),]);
    let points = HashMap::from([("X", 1),("Y", 2),("Z", 3),]);
    let to_pick_victory = HashMap::from([("C", "X"),("A", "Y"),("B", "Z"),]);
    let to_pick_loss = HashMap::from([("C", "Y"),("A", "Z"),("B", "X"),]);

    let mut count: i32 = 0;
    
    if actions[1].eq("Y") {
        count += points.get(*to_pick_draw.get(actions[0]).unwrap()).unwrap() + 3;
    } else if actions[1].eq("Z") {
        count += points.get(*to_pick_victory.get(actions[0]).unwrap()).unwrap() + 6;
    } else {
        count += points.get(*to_pick_loss.get(actions[0]).unwrap()).unwrap();
    }
    count
}