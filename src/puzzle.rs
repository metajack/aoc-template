use anyhow::Result;

pub trait Puzzle: Sized {
    fn from_input(input: &str) -> Result<Self>;
    fn solve_part1(&self) -> Result<String>;
    fn solve_part2(&self) -> Result<String>;
}
