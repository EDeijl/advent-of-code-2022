#[macro_use]
extern crate num_derive;

use crate::challenges::*;


mod challenges;
mod tools;
fn main() {
    println!("Hello, world!");
    day1::day_1_part_1();
    day1::day_1_part_2();

    day2::puzzle();
}
