//Rust code that takes an integer input from the terminal and prints out numbers from 0 to the input integer using a for loop
use std::io;

fn main() {
    println!("Enter an integer:");

    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let num: i32 = input.trim().parse()
        .expect("Please enter a valid number!");

    println!("Printing numbers from 0 to {}:", num);
    for i in 0..=num {
        println!("{}", i);
    }
}
