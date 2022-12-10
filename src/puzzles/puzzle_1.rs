use crate::puzzles::utils;

pub fn run_puzzle_1() {
    let input: String = utils::read_file("input/input_1.txt");

    let mut count: i32 = 0;
    let mut top3: Vec<i32> = vec![0, 0, 0];

    for line in input.lines() {
        if line.eq("") {
            if count > top3[0] {
                top3[2] = top3[1]; 
                top3[1] = top3[0];
                top3[0] = count;
            } else if (count < top3[0]) & (count > top3[1]) {
                top3[2] = top3[1];
                top3[1] = count;
            } else if (count < top3[0]) & (count < top3[1]) & (count > top3[2]) {
                top3[2] = count;
            }
            count = 0;
        }
        else {
            count += line.parse::<i32>().unwrap()
        }
    }

    let max: i32 = top3[0];
    let max_sum_of_3: i32 = top3.iter().sum();

    println!("------------------------");
    println!("Puzzle 1:");
    println!("Max: {0},\nSum of top 3: {1}", max, max_sum_of_3);
    println!("------------------------");
}
