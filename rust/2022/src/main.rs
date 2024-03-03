#[macro_use]
extern crate lazy_static;
extern crate reqwest;
mod input;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;

fn main() {
    day06::puzzle();
}
