use clap:: {
    Parser,
};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct AocArgs {
    /// The day of which you want to run the puzzle for.
    pub puzzle_day: String,
}