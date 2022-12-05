use crate::puzzles::utils;

pub fn run_puzzle_5() {
    let input: String = utils::read_file("input/input_5.txt");

    let (stack, moves) = input.split_once("\n\n").unwrap();

    let mut stacks_1: Vec<Vec<char>> = get_stacks(stack); 
    let mut stacks_2: Vec<Vec<char>> = get_stacks(stack);


    for move_ in moves.lines() {
        
        let mut split = move_.split(" ");
        let amount = split.nth(1).unwrap().parse::<usize>().unwrap();
        let from = split.nth(1).unwrap().parse::<usize>().unwrap();
        let to = split.nth(1).unwrap().parse::<usize>().unwrap();

        // Part 1
        (0..amount).for_each(|_| {
            let from = stacks_1[from - 1].pop().expect("stack empty.");
            stacks_1[to - 1].push(from);
        });

        // Part 2
        let amount = stacks_2[from - 1].len() - amount..;
        let crates: Vec<char> = stacks_2[from - 1].drain(amount).collect();
        stacks_2[to - 1].extend(crates);

    }
    let result_1: Vec<char> = stacks_1.iter().map(|f| f[f.len() - 1]).collect();
    let result_2: Vec<char> = stacks_2.iter().map(|f| f[f.len() - 1]).collect();

    println!("Puzzle 5:");
    println!("Solution 1: {:?},\nSolution 2: {:?}", result_1, result_2);
    println!("------------------------");
}

pub fn get_stacks(stack: &str) -> Vec<Vec<char>> {
    let width = (stack.lines().rev().next().unwrap().len() + 1) / 4;
    let mut result = Vec::new();
    for _ in 0..width {
        result.push(Vec::new())
    }
    stack.lines().rev().skip(1).for_each(|l| {
        let mut crates = l.chars().skip(1).step_by(4);
        result.iter_mut().for_each(|e| {
            if let Some(c) = crates.next() {
                if c.is_alphabetic() {
                    e.push(c)
                }
            }
        });
    });
    result
}

