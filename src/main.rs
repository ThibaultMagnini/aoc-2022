mod puzzles;
mod args;

use args::AocArgs;
use clap::Parser;

fn main() {
    let args = AocArgs::parse();
    match args.puzzle_day.as_str() {
        "01" => puzzles::puzzle_1::run_puzzle_1(),
        "02" => puzzles::puzzle_2::run_puzzle_2(),
        "03" => puzzles::puzzle_3::run_puzzle_3(),
        "04" => puzzles::puzzle_4::run_puzzle_4(),
        "05" => puzzles::puzzle_5::run_puzzle_5(),
        "06" => puzzles::puzzle_6::run_puzzle_6(),
        "07" => puzzles::puzzle_7::run_puzzle_7(),
        "09" => puzzles::puzzle_9::run_puzzle_9(),
        "10" => puzzles::puzzle_10::run_puzzle_10(),
        _ => panic!("The puzzle of day {} has not been created yet.", args.puzzle_day)
    }
}
