/*
* std - the standard library.
* io - input/output module.
* use - brings std::io into the scope.
* let - declares a variable. by default variables are immutable in rust.
* mut - allows a variable to be mutable.
* guess - the name of the variable
* String - variable type
* ::new() - an associated function of the String type.
* io::stdin - a function from the io module
* .read_line - a method on io
* &mut - & significes a reference to the guess variable.
*
* notice how `.read_line()` returns a Result<usize, Error>, an enum.
* a Result has a `.expect()` method, and handles the `Err`.
*
* rand is another library, with the Rng "trait".
*
* the book example uses a different depricated method off `rand`. instead, i used rng(), which
* expects an i32. but, `read_line` returns a String, so we need to trim, parse, and handle `Err`
*
* match is an expression, made up of arms. an arm consists of a pattern to match against.
* Ordering is a result type of `cmp`
*
* match expects you to pattern match against the result type, where `expect` expects you to handle
* any thrown non-success.
*
* */

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number: u32 = rand::rng().random_range(1..=100);

    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {guess}");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
