#![allow(unused)]

use std::fmt::Display;
use std::io;
use rand::Rng;
use std::io::{Write, BufRead, BufReader, ErrorKind};
use std::fs::File;
use std::fs;
use std::cmp::Ordering;
//Trait
use std::ops::Add;

use std::collections::HashMap;
use std::any::type_name;

use std::thread;
use std::time::Duration;

use std::rc::Rc;
use std::cell::RefCell;
use std::sync::{Arc, Mutex};

//TUtorial 24 -> modules
mod restaurant;
use crate::restaurant::order_food;

fn main() {
    println!("#################################");
    tut_30_2();
    println!("#################################");
	// let mut t: Vec<i32> = vec![11,12,12,23,34];
	// println!("{}",remove_dup(&mut t));
}

fn tut_30_2() {
    //RcT => Arc and Mutex, locks
    struct Bank {
        balance: f32
    }
    fn withdraw(the_bank:&Arc<Mutex<Bank>>, amt:f32) {
        let mut bank_ref = the_bank.lock().unwrap();
        if bank_ref.balance < amt {
            println!("Current balance is {}. Withdraw Smaller amount than {}.", bank_ref.balance, amt);
        } else {
            bank_ref.balance -= amt;
            println!("Withdrew {} from account. Remaining balance is {}.", amt, bank_ref.balance);
        }
    }
    fn customer(the_bank: Arc<Mutex<Bank>>) {
        withdraw(&the_bank, 5.0);
    }
    let bank: Arc<Mutex<Bank>> = Arc::new(Mutex::new(Bank{balance:20.0}));
    let handles = (0..10).map(|_| {
        let bank_ref = bank.clone();
        thread::spawn(|| {
            customer(bank_ref);
        })
    });
    for handle in handles {
        handle.join().unwrap();
    }
    // for i in 0..10{
    //     let bank_ref = bank.clone();
    //     thread::spawn(|| {
    //         customer(bank_ref);
    //     }).join().unwrap();
    // }
    println!("Total amount after withdrawls is {}", bank.lock().unwrap().balance);
}

fn tut_30_1() {
    //RcT -> a smart pointer that can allow multiple owners of variable,
    // if thread accesses var after main thread dosent exist, then its accessing 
    // a var that gets wiped along with main func.
    struct Bank {
        balance: f32
    }
    fn withdraw(the_bank: &mut Bank, amt:f32) {
        the_bank.balance -= amt;
    }
    let mut bank: Bank = Bank {
        balance: 100.0
    };
    withdraw(&mut bank, 5.0);
    println!("Bank balance after withdraw - {}",bank.balance);
    fn customer(the_bank: &mut Bank) {
        withdraw(the_bank, 10.0);
    }
    //will throw error,saying bank is accessed by thread that lives longer that main
    //but belongs to main.
    // thread::spawn(|| {
    //     customer(&mut bank);
    // }).join().unwrap();
}

fn tut_30() {
    //Concurrency and threads
    /*
        problems with parallel programing
        1. Threads access data in wrong order.
        2. Threads are blocked from executing because of confusion
           over requirements to proceed with execution.
     */
    let spawned_thread = thread::spawn(|| {
        for i in 0..20 {
            println!("Spawned thread - {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 0..15 {
        println!("Main thread - {}", i);
        thread::sleep(Duration::from_millis(1));
    };
    
    //To let spawned thread keep executing even after main thread stops.
    spawned_thread.join().unwrap();
}

fn tut_29() {
    //Smart pointers, Box
    /*
        Smart pointer is used to store the address of a variable,
        also with some functionality eg &str_var
     */
    // Box creates huge variables on the heap, passes its pointer to stack
    let var_1 = Box::new(10);
    let var_2 = Box::new("asd");

    struct TreeNode<T> {
        pub left: Option<Box<TreeNode<T>>>,
        pub right: Option<Box<TreeNode<T>>>,
        pub key: T
    }
    impl<T> TreeNode<T> {
        pub fn new(key: T) -> Self {
            TreeNode {
                left: None,
                right: None,
                key
            }
        }
        pub fn left(mut self, node: TreeNode<T>) -> Self {
            self.left = Some(Box::new(node));
            self
        }
        pub fn right(mut self, node: TreeNode<T>) -> Self {
            self.right = Some(Box::new(node));
            self
        }
    }
    let node_1 = TreeNode::new(32);
    node_1.left(TreeNode::new(12)).right(TreeNode::new(43));
    //println!("node_1.key val - {}",&node_1.key);
}
/*
    leetcode
*/
fn str_str(needle: String, haystack: String) ->i32 {
	
	0
}

fn remove_dup(nums: &mut Vec<i32>) -> i32 {
	let mut res:Vec<i32> = vec![];
	for i in nums.iter() {
		if !res.contains(i) {
			res.push(*i);
		}
	}
	*nums = res.clone();
	return res.len().try_into().unwrap();
}

fn tut() {
	let mut a = vec![1,2,3];
	a.push(4);
	a.is_empty();
	match a.pop() {
		Some(4) => println!("pop 1"),
		_ => println!("not poped")
	}
	for x in a.iter() {
		println!("{}",x);
	}
	let r = a.len() as i32;
}
/*
    leetcode ends here
*/

fn tut_28() {
	//Closures -> function without a name, can be passed as arguments to other functions
	//can be stored in a variable
	//let var_name = |parameters| -> return_type {
	//	BODY
	//}
	let can_vote = |age: i32| {
		age >= 18
	};
	println!("Can vote - {}", can_vote(24));

	//Closures can access variables outside its scope, it can also modify if its of mut type
	let mut samp1 = 5;
	// no parameters -> ||
	let print_samp = || println!("samp1 - {}", samp1);
	print_samp();
	samp1 = 10;
	let mut change_samp = || samp1 += 1;
	//print_samp(); -> gives error
	change_samp();
	println!("samp1 - {}", samp1);
	samp1 = 10;
	println!("samp1 - {}",samp1);

	//Closures as parametes to functions
	//use_func is generic, takes generic type of function
	fn use_func<T>(a:i32, b:i32, func: T) -> i32 
	where T: Fn(i32,i32)->i32 {
		func(a, b)
	}
	let sum = |a,b| a+b;
	let prod = |a,b| a*b;
	println!("4+3 = {}",use_func(4, 3, sum));
	println!("4*3 = {}",use_func(4, 3, prod));
}

fn tut_27() {
	//Iterators
	let mut arr_iter = [1,2,3,4];
	for val in arr_iter.iter() {
		println!("{}",val);
	}
	// consumes old array, previous invalid
	// let mut tt = arr_iter.into_iter();
	// let is_true = tt.all(|x| x>0);
	let mut iter1 = arr_iter.iter();
	println!("!st item is - {}",iter1.next().unwrap()); 

}

fn file_tut_remove() {
	let path = "src/test_dir";
	fs::create_dir(path).unwrap();
	let mut ip = String::new();
	io::stdin().read_line(&mut ip);
	let close = ip.trim();
	println!("{}", ip.as_str());
	println!("{}", ip);
	match close.cmp("y") {
		Ordering::Equal => fs::remove_dir_all(path).unwrap(),
		_ => println!("invalid Input"),
	}
}

fn tut_26() {
    //FILE IO and Result<T,E>, unwrap(), ErrorKind
	let path = "src/files_created";
	match fs::create_dir(path) {
		Ok(()) => (),
		Err(e) => match e.kind() {
			ErrorKind::AlreadyExists => {
				fs::remove_dir_all(path).unwrap();
				fs::create_dir(path).unwrap();
			},
			_ => panic!("Error creating file - {}", e)
		}
	};
    let path = "src/files_created/lines.txt";
    let output = File::create(path);
    let mut output = match output {
        Ok(file) => file,
        Err(err) => {
            panic!("Error creating file - {:?}", err);
        }
    };
    write!(output, "line1 text\n some more text line2\nline3")
        .expect("Error writing to the file");
    //.unwrap returns the value directly, instead of Result<> and panics if err
    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);
    for line in buffered.lines() {
        println!("{}", line.unwrap());
    }
    //ErrorKind tutorial -> Enum with all error kinds
    let output_2 = File::create("src/files_created/rnd_python.py");
    let output_2 = match output_2 {
        Ok(file) => file,
        Err(err) => {
            match err.kind() {
				ErrorKind::NotFound => {
					match File::create("src/files_created/rnd_python.py") {
						Ok(file) => file,
						Err(err) => panic!("Error creating py file - {:?}", err)
					}
				},
				_other_err => {
					panic!("Error creating file - {:?}", err);
				}
            }
        }
    };
	let mut remove_file = String::new();
	println!("Shall we remove files_created directory? Y/N");
	io::stdin().read_line(&mut remove_file);
	let close = remove_file.trim();
	match close {
		"Y"|"y" => match fs::remove_dir_all("src/files_created") {
			Ok(()) => println!("files_created dir removed"),
			Err(err) => panic!("Error removing files_created - {}", err)
		},
		_ => return
	}
}

fn tut_25() {
    //Error Handling
    /*  No exceptions in rust, 
        Recoverable errors are handled with Result<Ok, Err>, like subscribe in JS
        unrecoverable errs are handled with panic macro, panic!(), it prints message and program quits
     */
    /*
        Result has 2 variants Ok and Err  
        enum Result<T,E> {
            Ok(T),
            Err(E)
        }
        //here T is datatype of variable to return, and E is type of error
     */
    //panic!("Error occured, quiting program now");
    // let arr = [1,2,3,4];
    // println!("10th indec in arr {}", arr[10]);
}

fn tut_24() {
    /*
        Modules in Rust, crate -> rust file with some code
        package contains multiple crates
        definitions -
        Crate - Modules that produce a library or an executable
        Modules -: Organise and handle privacy
        Packages -: Build, test and share crates
        Paths -: A way of naming an item such as a struct or function
     */
    //added folder structure in src/restaurant as demo
    order_food();
}

fn tut_23() {
    //Traits -> for adding functions to structs,
    //They like interfaces in OOPs concepts, 
    trait Shape {
        //constructor
        fn new(length: f32, width: f32) -> Self;
        fn area(&self) -> f32;
    }
    struct Rectangle {
        length: f32,
        width: f32
    }
    struct Circle {
        length:f32,
        width: f32
    }
    impl Shape for Rectangle {
        fn new(length: f32, width: f32) -> Rectangle {
            return Rectangle {
                length,
                width
            }
        }
        //area gives l*b
        fn area(&self) -> f32 {
            return self.length * self.width;
        }
    }
    const PI: f32 = 3.1415926;
    impl Shape for Circle {
        fn new(length: f32, width: f32) -> Circle {
            return Circle {
                length,
                width
            }
        }
        // area gives pi *r *r
        fn area(&self) -> f32 {
            return (self.length/2.0).powf(2.0) * PI
        }
    }
    let rect:Rectangle = Shape::new(3.0,4.0);
    let circ:Circle = Shape::new(4.0,4.0);
    println!("Area of rectangle - {}", rect.area());
    println!("Area of circle - {}", circ.area());
}

fn tut_23_1() {
    //Traits with generics //self excercise
    //Simulating OOP inheritance behaviour, getters and setters
    trait Organism<N,A> {
        fn new(name: N, age: A) -> Self;
        fn get_age(&self) -> &A;
        fn get_name(&self) -> &N;
        fn set_age(&mut self,age:A);
    }
    struct Dog<N, A> {
        name: N,
        age: A
    }
    struct Horse<N, A> {
        name:N,
        age:A
    }
    impl<N,A> Organism<N,A> for Dog<N,A> {
        fn new(name: N, age: A) -> Dog<N,A> {
            return Dog {
                name,
                age
            }
        }
        fn get_age(&self) -> &A {
            &self.age
        }
        fn get_name(&self) -> &N {
            &self.name
        }
        fn set_age(&mut self,new_age:A) {
            self.age = new_age;
        }
    }
    impl<N,A> Organism<N,A> for Horse<N,A> {
        fn new(name: N, age: A) -> Horse<N,A> {
            return Horse {
                name,
                age
            }
        }
        fn get_age(&self) -> &A {
            &self.age
        }
        fn get_name(&self) -> &N {
            &self.name
        }
        fn set_age(&mut self, new_age:A) {
            self.age = new_age;
        }
    }
    let mut horse:Horse<&str, i32> = Organism::new("Bojack Horseman", 32);
    let mut dog:Dog<&str, &str> = Organism::new("Mr. Peanutbutter", "sweet sixteen");

    println!("Age of horse - {}", horse.get_age());
    println!("Age of dog - {}", dog.get_age());
    dog.set_age("its 24");
    horse.set_age(43);

}

fn tut_22() {
    //Struct -> Custome Datatype
    struct Titan {
        name: String,
        size: i32,
        holder_name: String,
        hostile_to_eldia: bool
    }
    let mut cart_titan = Titan {
        name: String::from("Cart Titan"),
        size: 4,
        holder_name: String::from("Peick"),
        hostile_to_eldia: true
    };
    println!("Cart titan's height is {} meters.", cart_titan.size);
    cart_titan.hostile_to_eldia = false;

    //Structs using geneics, like functions
    struct Rectangle<T, K> {
        length: T,
        breadth: K
    }
    let rectangle = Rectangle {
        length: 23,
        breadth: 43.4
    };
    let rectangle_prank = Rectangle {
        length: "sike",
        breadth: "nigga"
    };

    //structs as generic type //complex struct
    struct Parent<T1, T2> {
        child_1:T1,
        child_2:T2
    }

    let parent_struct = Parent {
        child_1: Rectangle {
            length: 20,
            breadth: "Wont Say"
        },
        child_2: Titan {
            name: String::from("Attach Titan"),
            size: 15,
            holder_name: String::from("Eren iyeagah"),
            hostile_to_eldia: false
        }
    };
    println!("parent struct props  =  {}", parent_struct.child_1.breadth);
    
}

fn tut_21() {
    //HashMaps
    let mut heroes: HashMap<&str, &str> = HashMap::new();
    heroes.insert("tatakae", "eren yeagah");
    heroes.insert("erehh", "mikasa");
    heroes.insert("shinzosasageyoo", "ErivinDanjooo");

    for (k,v) in heroes.iter() {
        println!("{} says {}", v, k);
    }
    println!("length of HashMapuu is {}", heroes.len());
    if heroes.contains_key("erehh") {
        let mikasa_value = heroes.get("erehh");
        match mikasa_value {
            Some(x) => println!("MIkasa says erehh is in hashmapuu"),
            None => println!("Mikasa not found in hashmapuu")
        }
    }
}

fn tut_20() {
    //Ownership
    // stack -> memory for variables of fixed size, stores in last in first out manner
    // Heap -> When putting data on heap, we request a certain amount of mem
    // from the OS, it finds space and gives the address of that location back
    // called pointer

    //RULES OF OWNERSHIP
    //1. Each value has variable that's its owner.
    //2. There is only one owner at a time for a value.
    //3. When the owner goes out of scope, its value dissapears.
    let mut str_1 = String::from("Hello");
    let str_2 = str_1.clone(); // Will work if str_1.clone(), println
    //println!("{}",str_1); //error
    /*
        str_3 = str_1 gives error, coz String "Hello" is a val, has one owner str_1 var
        then str_2 becomes its owner, and str_1 is moved/removed.
        so when assigning to str_3, str_1 is not owner of val "hello", so can't give what is dosent own
     */
    //let str_3 = str_1;
    print_str(str_2.clone());// below line wont work if cloned, as ownership goes to function
    let ret_str = return_str(str_2);
    change_str(&mut str_1);
    println!("str_1 after change function - {}",str_1);
}

fn print_str(x: String) {
    println!("String is - {}", x);
}

fn return_str(x: String) -> String {
    println!("String to return - {}",x);
    x
}

fn change_str(x: &mut String) {
    x.push_str(" is happy. Finally!");
    println!("Changed String is - {}", x);
}

// End of tut_20

fn tut_19() {
    //Generics, funtions that specify the datatypes of variables at a later time
    //Functions take params and returns any data type
    println!("Generics, same function for multiple data types");
    println!("int vals {} + {} = {}", 5, 6, get_sum_gen(5,6));
    println!("float values {} + {} = {}", 4.3, 5.4, get_sum_gen(4.3, 5.4));
}

//using Traits Add -> for + operator
fn get_sum_gen<T:Add<Output = T>>(x: T, y: T) -> T {
    return x + y;
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
