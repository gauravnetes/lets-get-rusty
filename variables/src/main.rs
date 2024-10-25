use std::{iter::Product, result};

fn main() {
    // variables:

    let mut x = 5;  // variables in rust are immutable by default
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // constants are not mutable
    const SUBSCRIBER_COUNT: u32 = 100_000;

    let y = 4;   // immutable
    println!("y: {}", y); 
    let y = "four"; // immutable
    println!("y: {}", y); 

    // datatypes: Four Scalar Datatypes => Integers, Floating-point numbers, Booleans, Characters
    let a = 98_333;        // decimal
    let b = 0xff;          // hex
    let c = 0o77;          // octal
    let d = 0b1111_0000;   // Binary
    let e = b'A';           // Byte

    let f: u8 = 255; // u8 has the max capacity of 255. greater than this rust build will break

    // floating-point numbers
    let f = 2.0;
    let g: f32 = 3.0;

    // operations:
    let sum = 5 + 10; 
    let difference = 95.5 - 4.3;
    let product = 4 * 30; 
    let quotient = 87.3 / 3.4; 
    let remainder = 53 % 9;  

    // booleans: 
    let t = true;  
    let f = false; 

    // characters:
    let c = 'c'; 

    // compound types:
    let tup = ("rusty rust", 100_000); // fixed type array with related data that could be of different type
    // to access value inside tuple:
    let (channel, sub_count) = tup; // channel = rusty rust, sub_count = 100_000
    let sub_count = tup.1;

    // tuple and array both have 0th indexing in rust

    // arrays:
    let error_codes = [200, 404, 500];  // array size are fixed in rust. [dynamic => vector] 
    let not_found = error_codes[1]; // 404

    let byte = [0; 8]; 

    // functions: 
     let sum1 = my_function(4, 5);
     println!("Sum: {}", sum1);



     // control flow: 
     let num = 5;
     if num < 10 {
        println!("first condition true");
     } else if num < 22 {
        println!("second condition true"); 
     } else {
         println!("condition false"); 
     }
     // Another syntax: 
     let condition = true; 
     let number = if  condition {5} else {6}; // condition: true => num: 5 / false => num: 6

     // loops:
     let mut counter = 0; 

     let result = loop {
        counter += 1;
        if counter == 10 {
            break counter;
        } 
     };
     println!("result: {}", result);

     // while loop: 
     let mut number = 3; 

     while number != 0 {
         println!("{}!", number); 
         number -= 1; 
     }
     println!("LIFTOFF");
 
     // for/ for-in loop: used for set of data or collection 
     let a = [10, 20, 30, 40, 50];
     for element in a.iter() {
        println!("value: {}", element);  // for every element in the array[a] take the element and print it 
     }

     for number in 1..4 {
        println!("{}!", number);  // print the numbers 1 to 3
     }
}

// functions: 
fn my_function (p: i32, q: i32) -> i32{   // to specify the return type -> is the action
    println!("value of p: {} ", p);
    println!("value of q: {} ", q);
    let sum = p + q; 
    sum // last expression is implicitly returned inside a funciton so the return keyword isn't necessary 
}
// rust uses snake case convention. the function name should be in small letters