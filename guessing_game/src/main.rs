use rand::Rng; // to set the range of the random generated number
use std::cmp::Ordering; // to compare the guessed and the secret number
use std::io; // use std::io to read input, trim()
use colored::*;// use colored crate to print text in colors
fn main() {
    println!("Guess The Number:");

    let secret_num = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {} ", secret_num);

    loop {
        println!("Input your guess");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read Line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_num) {
            // compare the guessed number to the randomly generated secret number
            Ordering::Less => println!("{}", "Too low".red()),
            Ordering::Greater => println!("{}", "Too High".red()),
            Ordering::Equal => {
                println!("{}", "You guessed the Number. Big W".green());
                break;
            },
        }
    }
}

// 1. use rand crate to generate a random number
// 2. use std::io to read input, trim()
// 3. convert a variable to another type using something like :u32
// 4. use std::cmp compare two variables
// 5. loop(continue+break)
// 6. match
// 7. Result type
// 8. use colored crate to print text in colors