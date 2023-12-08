mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

use clap::ValueEnum;
use self::{day1::day1, day2::day2, day3::day3, day4::day4, day5::day5};



#[derive(ValueEnum, Debug, Clone, Copy)]
pub enum Days {
    Day1 = 1,
    Day2 = 2,
    Day3 = 3,
    Day4 = 4,
    Day5 = 5
}

impl Days {
    pub fn solve(self, input: String, part: i32) -> i64 {
        match self {
            Days::Day1 => day1(input, part),
            Days::Day2 => day2(input, part),
            Days::Day3 => day3(input, part),
            Days::Day4 => day4(input, part),
            Days::Day5 => day5(input, part)
        }
    }
}


