#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufRead, BufReader, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    tut3();
}

fn tut3() {
    // unsigned integers: u8, u16, u32, u64, u128, usize
    // signed integers: i8, i16, i32, i64, i128, isize
    println!("Max u32: {}", u32::MAX);
    println!("Max u64: {}", u64::MAX);
    println!("Max u128: {}", u128::MAX);
    println!("Max usize: {}", usize::MAX);
}

fn tut2() {
    const ONE_MIL: u32 = 1_000_000;
    const PI:f32 = 3.141592;
    let age:&str = "47";
    let mut age:u32 = age.trim().parse()
        .expect("Age is not a number");
    age += 1;
    println!("I'm {} years old, I want Rs.{}", age, ONE_MIL);

}

fn tut1() {
    println!("Thou shall be known as what?");
    let mut name = String::new();
    let greeting = "Namaste 🙏🙏🙏";
    io::stdin().read_line(&mut name)
        .expect("Your name hasn't pleased though toungue, Repeat!");
    println!("Hello, {}! {}",name.trim_end() ,greeting);
}
