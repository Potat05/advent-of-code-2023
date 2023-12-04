mod day1;
mod day2;
mod day3;

use clap::ValueEnum;
use self::{day1::day1, day2::day2, day3::day3};



#[derive(ValueEnum, Debug, Clone, Copy)]
pub enum Days {
    Day1 = 1,
    Day2 = 2,
    Day3 = 3
}

impl Days {
    pub fn solve(self, input: String, part: i32) -> i32 {
        match self {
            Days::Day1 => day1(input, part),
            Days::Day2 => day2(input, part),
            Days::Day3 => day3(input, part)
        }
    }
}


