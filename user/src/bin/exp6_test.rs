#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::{get_time, yield_};

#[no_mangle]
fn main() -> i32 {
    println!("exp6_test start");
    for round in 0..6 {
        let now = get_time();
        println!("exp6_test round {} time {}", round, now);
        yield_();
    }
    println!("Test exp6_test OK!");
    0
}
