use crate::puzzles::utils::read_file;

pub fn run_puzzle_1() {
    let input: String = read_file("input/input_1.txt");

    let mut count: i32 = 0;
    let mut result: Vec<i32> = Vec::new();

    for line in input.lines() {
        if line.eq("") {
            result.push(count);
            count = 0;
        }
        else {
            count += line.parse::<i32>().unwrap()
        }
    }

    let l: usize = result.len();

    result.sort();

    let max: i32 = result[l - 1];
    let max_sum_of_3: i32 = result[l - 1] + result[l - 2] + result[l - 3];
    
    println!("Max: {0}, Sum of top 3: {1}", max, max_sum_of_3);
}
