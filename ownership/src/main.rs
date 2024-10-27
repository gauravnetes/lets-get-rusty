use std::io::Bytes;

fn main() {
    // ---- Ownership Rules----
    // 1. Each value in rust has a variable which is called it's owner
    // 2. There can only be one owner at a time
    // 3. When the owner goes out of scope, the value will be dropped.
    {
        // s is not valid here as it's not declared
        let s = String::from("hello"); // s is valid from this point forward
                                       // use s in the scope
    } // s is now over with the scope

    let x = 5;
    let y = x; // copy

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("{}, World", s1);

    // ownership and functions:
    let s4 = String::from("hello");
    takes_ownership(s4);
    //  println!("{}", s4); // s4 no longer has the ownership so it'll show error

    let s3 = gives_ownership();
    println!("s3: {}", s3); // s3 has the ownership

    // take ownership and give it back
    let s5 = gives_ownership();
    let s6 = String::from("hello");
    let s7 = takes_and_gives_back(s1);
    println!("s5: {}, s6: {}", s5, s7);

    // References: [if we don't want to use the OWNERSHIP model]
    let s9 = String::from("hello");
    let len = calculate_length(&s9); // passing a reference to the string. references don't take the ownership
    println!("The length of {} is {}", s2, len);

    let mut s10 = String::from("hello");
    change(&mut s10);

    mutable();

    let mut s13 = String::from("hello world");
    // we've to manually keep a return value in sync with the string. which is error prone
    // for the solution there are "Slices"
    let hello = &s13[0..5]; // part of the string starting from index 0 and ending at index 4. 5 will be excluded
    let world = &s13[6..11]; 
    let entire = &s13[..]; // entire will get the entire entire string "hello world"
    let word = first_word(&s13); 
    s13.clear();

    // slice on array:
    let a = [1, 2, 3, 4, 5]; 
    let slice = &a[0..2]; // will store form 1 to 2 [0th to 1st index]
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn gives_ownership() -> String {
    let another_string = String::from("hello");
    another_string // returning the string moves the ownership of the string to the s3 variable
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

// reference:
fn calculate_length(s: &String) -> usize {
    // we're taking a reference to the string
    // references are immutable by default. we can't borrow the values as mutable.
    let length = s.len();
    length
}

// to make it mutable without the ownership
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// NOTE: Mutable references: You have one mutable reference to a particular piece of data to a particular scope
// you can't have mutable reference if immutable reference already exists.

fn mutable() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;

    println!("{}, {}", r1, r2);

    let r3 = &mut s;
    println!("{}", r3);
}

// slices: don't take ownership of the underlined data
fn first_word(s: &String) -> &str{
    let bytes = s.as_bytes();

    for (i, &item) in  bytes.iter().enumerate() {
        if item == b' ' {
            return  &s[0..i];
        }
    }
    &s[..]  // return the length of the string if spaces are found
}