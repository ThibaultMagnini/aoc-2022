use crate::puzzles::utils;

pub fn run_puzzle_10() {
    let input: String = utils::read_file("input/input_10.txt");

    let mut register = 1;
    let mut cycle = 0;
    let mut strength = 0;

    for line in input.lines() {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        cycle += 1;
        if (cycle - 20) % 40 == 0 {
            strength += cycle * register;
        }
        draw(cycle, register);

        match tokens[0] {
            "noop" => {},
            "addx" => {
                cycle += 1;
                if (cycle - 20) % 40 == 0 {
                    strength += cycle * register;
                }
                register += tokens[1].parse::<i32>().unwrap();
                draw(cycle, register);
            },
            _ => unreachable!()
        }
    }
    println!("------------------------");
    println!("\nPuzzle 10:");
    println!("Solution 1: {}", strength);
    println!("------------------------");
}

fn draw(cycle: i32, register: i32) {
    let x_pos = cycle % 40;
    if register == x_pos || register == x_pos - 1 || register == x_pos + 1 {
        print!("#");
    } else {
        print!(" ");
    }
    if x_pos == 39 {
        print!("\n");
    }
}