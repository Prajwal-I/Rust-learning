#![allow(unused)]

use std::fmt::Display;
use std::io;
use rand::Rng;
use std::io::{Write, BufRead, BufReader, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    println!("############################");
    tut_18();
    println!("############################");
}

/*
    TUT 18 Functions ---start
 */
fn tut_18() {
    //Functions
    say_hello();
    get_sum(3,4);
    println!("{}",get_sum_2(4,5));
    println!("{}",get_sum_return(42,5));

    let (val_1, val_2) = get_2_vals(5);
    println!("Returned values 1-> {}, 2-> {}", val_1, val_2);
    let list_ele = vec![1,2,3];
    println!("sum of all list_ele is - {}", sum_list(&list_ele));
}

fn sum_list(list: &[i32]) -> i32 {
    /*
        A function that has a parameter as a reference to a vector array
        that returns the sum of all the elements of that array.
     */
    let mut sum = 0;
    for &i in list.iter() {
        sum += &i;
    }
    return sum;
}

fn get_2_vals(x: i32) -> (i32, i32) {
    // A function that returns multiple values
    return (x+1 , x+2);
}

fn get_sum_return(x: i32, y: i32) -> i32 {
    println!("{} + {} = {}", x, y, x+y);
    return x + y;
}

fn get_sum_2(x: i32, y: i32) -> i32 {
    println!("{} + {} = {}", x, y, x+y);
    //can return without return keyword, but no ;
    x + y
}

fn get_sum(x: i32, y: i32) {
    println!("{} + {} = {}", x, y, x+y);
}

fn say_hello() {
    println!("Hello world");
}
/*
    TUT 18 Functions ---End
 */

fn tut_17() {
    //Vectors
    let vec_1: Vec<i32> = Vec::new();
    println!("Vec_1 size = {}",vec_1.len());
    //vec_1.push('a' as i32);
    let mut vec_2: Vec<i32> = vec![1,2,3,4];
    vec_2.push(5);
    println!("1st ele of vec_2 - {}", vec_2[1]);
    let second: &i32 = &vec_2[1];
    println!("second - {}", second);
    match vec_2.get(1) {
        Some(second) => println!("2nd ele of vec_2 - {}", second),
        None => println!("No value at 2nd element of vec_2")
    }
    for i in &mut vec_2 {
        *i *= 2;
    }
    for i in &vec_2 {
        println!("vec_2 value - {}", i);
    }
    println!("Length of vec_2 - {}", vec_2.len());
    println!("Pop: {:?}", vec_2.pop());
}

fn tut_16() {
    //enums
    enum Day {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday
    }

    impl Day {
        fn is_weekend(&self) -> bool {
            match self {
                Day::Saturday | Day::Sunday => true,
                _ => false
            }
        }
    }

    let day:Day = Day::Monday;
    match day {
        Day::Monday => println!("Monday new begining"),
        Day::Tuesday => println!("Getting started"),
        Day::Wednesday => println!("Its wednesday my dudes"),
        Day::Thursday => println!("In two days"),
        Day::Friday => println!("dis friday"),
        Day::Saturday => println!("Tis weekend"),
        Day::Sunday => println!("Tis last weekend njoy like never before.")
    }
    println!("Is it weekend today - {}", day.is_weekend());
    println!("Is Weekday, - {}", !Day::is_weekend(&day));
}

fn tut_15() {
    //type casting
    let num_1:u8 = 12;
    let num_2:u8 = 34;
    let num_3:u32 = (num_1 + num_2) as u32;
    let num_4:u64 = (num_2 as u64) + (num_3 as u64);
    let ch:char = 68 as char; //ASCII for D = 68
    let alp: char = 'F';
    println!("num_3 - {}, num_4 - {}, ch - {}, alp UNI - {}",
     num_3, num_4, ch, alp as u16);
}

fn tut_14_1() {
    let str1 = String::from("a d w g e f");
    println!("Str1 = {}",str1);
    let mut v1: Vec<char> = str1.chars().collect();
    // for vc in v1 {
    //     println!("v1 content - {}",vc);
    // }
    v1.sort();
    //removes repeating chars
    v1.dedup();
    for vc in v1 {
        println!("v1 content sort & dedup - {}",vc);
    }
    let st2: &str = "Henlo";
    let mut st3 = st2.to_string();
    println!("st2 - {}, st3 - {}", st2, st3);
    let byte_arr_1 = st3.as_bytes();
    for bt in byte_arr_1 {
        println!("bt element = {}", (bt));
    }
    //copies string till specified index range
    let st4 = &st2[0..4];
    println!("st4 aplit index - {}", st4);
    println!("st4 len - {}",st4.len());
    st3.clear();
    println!("st3 after .clear() - {}", st3);
    //concatinating strings
    let st_1 = String::from("yooo");
    let st_2 = String::from("bruhhh");
    let st_test = String::from("testtt");
    let st_3 = st_1.clone() + &st_2 + &st_test;
    for ch in st_3.bytes() {
        println!("element in st_3 concatinated - {}", ch);
    }
    println!("st_1 after concatinating - {}", st_1);    
}

fn tut_14() {
    //Strings, String type -> vector of bytes, can be changed
    //&str -> referece, its pointing to string, used for viewing
    let mut st1 = String::new();
    st1.push('A');
    st1.push_str(" Word");
    //looping through string
    for wrd in st1.chars() {
        println!("{}",wrd);
    }
    //split
    for wrd in st1.split_whitespace() {
        println!("{}",wrd);
    }
    let st2 = st1.replace("A", "Another");
    println!("{}",st2);
}

fn tut_13() {
    //Tuples
    let tuple: (u32, String, f32) = (23, "Shyam".to_string(), 50_893.00);
    //using index to get value
    println!("Name: {}", tuple.1);
    //creating seperate variables from tuple
    let (v1, v2, v3) = tuple;
    println!("Age: {}", v1);
}

fn tut_12_2() {
    //for loop
    let arr = [1,2,3,4,5];
    for val in arr.iter() {
        println!("Array value {}", val);
    }
}

fn tut_12_1() {
    //While loop
    let arr = [1,2,3,4,5];
    let mut loop_idx = 0;
    while loop_idx < arr.len() {
        println!("Arr value at {} is {}", loop_idx, arr[loop_idx]);
        loop_idx += 1;
    }
}

fn tut_11_1() {
    //Looping through Array
    let arr_2 = [1,2,3,4,5,6,7,8,9];
    let mut loop_idx = 0;
    //loop key word for looping
    loop {
        if arr_2[loop_idx] % 2 == 0 {
            loop_idx += 1;
            continue;
        }
        // TO break loop at length of array
        //if loop_idx == arr_2.len()-1 { 
        if arr_2[loop_idx] == 9 {
            break;
        }
        println!("Val = {}", arr_2[loop_idx]);
        loop_idx += 1;
    }
}

fn tut_11() {
    //Arrays
    let arr_1 = [1,2,3,4];
    println!("1st Element = {}",arr_1[0]);
    println!("Length = {}", arr_1.len());
}

fn tut_10_1() {
    let my_age = 18;
    let voting_age = 18;
    //match has many built in functions, cmp is one, which compares values.
    match my_age.cmp(&voting_age) {
        Ordering::Less => println!("Can't Vote"),
        Ordering::Greater => println!("Can Vote"),
        Ordering::Equal => println!("You just turned legal! ;)")
    }
}

fn tut_10() {
    //Match (equivalent to switch case)
    // 1..3 indicated range from 1 to 2, 1..=3 includes 3 in range
    let my_age = 16;
    match my_age {
        (1..=18) => println!("Important Age"),
        21 | 50 => {
            println!("Important Age")
        },
        65..=i32::MAX => println!("Important Age"),
        // Default is written as _
        _ => println!("Unimportant Age")
    }

}

fn tut_9() {
    //ternary operator
    let my_age = 24;
    let can_vote = 
        if my_age > 18 {
            true
        } else {
            false
        };
    println!("Can vote = {}", can_vote);
}

fn tut_8() {
    //if else
    let age = 21;
    if (age>=1) && (age<=18) {
        println!("Important Birthday");
    } else if (age == 21) || age == 50 {
        println!("Important Birthday");
    } else if age >=65 {
        println!("Important Birthday");
    } else {
        println!("Unimportant Birthday");
    }
}

fn tut_7_random_num() {
    //random number 1 to 100
    let random_num = rand::thread_rng().gen_range(1..101);
    println!("Random number = {}", random_num);
}

fn tut_6() {
    //Arithmetic operations
    let num_1: u32 = 5;
    let mut num_2: u32 = 4;
    println!("5 + 4 = {}", num_1 + num_2);
    println!("5 - 4 = {}", num_1 - num_2);
    println!("5 * 4 = {}", num_1 * num_2);
    println!("5 / 4 = {}", num_1 / num_2);
    println!("5 % 4 = {}", num_1 % num_2);
    num_2 += 1;
    println!("num_2 + 1 = {}", num_2);
}

fn tut_5() {
    //Prescision of f32 vs f64
    let num_1:f32 = 1.111111111111111;
    println!("f32 : {}",num_1 + 0.111111111111111);
    let num_2:f64 = 1.111111111111111;
    println!("f64 : {}",num_2 + 0.111111111111111);
}

fn tut_4() {
    //data types
    let is_true = true;
    let my_grade = 'A';
    // to ignore not using variable warning by compiler
    let _is_true = false;
}

fn tut_3() {
    // unsigned integers: u8, u16, u32, u64, u128, usize
    // signed integers: i8, i16, i32, i64, i128, isize
    println!("Max u32: {}", u32::MAX);
    println!("Max u64: {}", u64::MAX);
    println!("Max u128: {}", u128::MAX);
    //usize for mutable variables, let mut a: usize = 2;
    println!("Max usize: {}", usize::MAX);
    println!("Max f32: {}", f32::MAX);
    println!("Max f64: {}", f64::MAX);
    println!("Max i64: {}", i64::MAX);
}

fn tut_2() {
    //Constants and parsing, inbuilt err haldling Err and Ok
    const ONE_MIL: u32 = 1_000_000;
    const PI:f32 = 3.141592;
    let age:&str = "47";
    let mut age:u32 = age.trim().parse()
        .expect("Age is not a number");
    age += 1;
    println!("I'm {} years old, I want Rs.{}", age, ONE_MIL);

}

fn tut_1() {
    //IO operations
    println!("Thou shall be known as what?");
    let mut name = String::new();
    let greeting = "Namaste 🙏🙏🙏";
    io::stdin().read_line(&mut name)
        .expect("Your name hasn't pleased though toungue, Repeat!");
    println!("Hello, {}! {}",name.trim_end() ,greeting);
}
