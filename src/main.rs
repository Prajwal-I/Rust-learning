#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufRead, BufReader, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    const ONE_MIL: u32 = 1_000_000;
    const PI:f32 = 3.141592;
    let age:&str = "47 ";
    let mut age:u32 = age.trim().parse()
        .expect("Age is not a number");
    age += 1;
    println!("I'm {} years old, I want Rs.{}", age, ONE_MIL);

    // println!("Thou shall be known as what?");
    // let mut name = String::new();
    // let greeting = "Namaste 🙏🙏🙏";
    // io::stdin().read_line(&mut name)
    //     .expect("Your name hasn't pleased though toungue, Repeat!");
    // println!("Hello, {}! {}",name.trim_end() ,greeting);
}
