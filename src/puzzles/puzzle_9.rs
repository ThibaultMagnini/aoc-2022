use crate::puzzles::utils;
use std::collections::HashSet;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

pub fn run_puzzle_9() {
    let input: String = utils::read_file("input/input_9.txt");

    let mut visited: HashSet<Pos> = HashSet::with_capacity(10_000);
    let mut visited2: HashSet<Pos> = HashSet::with_capacity(10_000);
    let mut head = Pos::default();
    let mut tails = [Pos::default(); 9];

    for line in input.lines() {
        let (direction, amount) = line.split_once(' ').unwrap();
        for _ in 0..amount.parse().unwrap() {
            match direction {
                "U" => head.y += 1,
                "D" => head.y -= 1,
                "L" => head.x -= 1,
                "R" => head.x += 1,
                _ => panic!("no valid direction"),
            }
            follow(&head, &mut tails[0]);

            visited.insert(tails[0]);
            
            for i in 1..9 {
                let (left, right) = tails.split_at_mut(i);
                follow(&left[i - 1], &mut right[0]);
            }
            visited2.insert(tails[8]);
            
        }
    }

    println!("------------------------");
    println!("Puzzle 9:");
    println!("Solution 1: {}\nSolution 2: {}", visited.len(), visited2.len());
    println!("------------------------");
}

fn follow(lead: &Pos, t: &mut Pos) {
    let dx = lead.x - t.x;
    let dy = lead.y - t.y;
    if (dx.abs() + dy.abs() > 1) && (dx.abs() > 1 || dy.abs() > 1) {
        t.x += dx.signum();
        t.y += dy.signum();
    }
}