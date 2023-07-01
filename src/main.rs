#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufRead, BufReader, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    println!("Thou shall be known as what?");
    let mut name = String::new();
    let greeting = "Namaste 🙏🙏🙏";
    io::stdin().read_line(&mut name)
        .expect("Your name hasn't pleased though toungue, Repeat!");
    println!("Hello, {}! {}",name.trim_end() ,greeting);
}
