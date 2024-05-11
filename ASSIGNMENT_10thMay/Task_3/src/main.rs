//Program to subtract, multiply, divide , add 2 to the input integer
use std::io;

fn main() {
    println!("Enter an integer:");

    
    let mut int = String::new();

    
    io::stdin().read_line(&mut int)
        .expect("Failed to read line");

    
    let int: i32 = int.trim().parse()
        .expect("Please enter a valid integer");

   
    let sub = int - 2;
    let mul = int * 2;
    let div = int / 2;
    let add = int + 2;

    
    println!("{} -  2: {}",int, sub);
    println!("{} *  2: {}",int, mul);
    println!("{} /  2: {}",int, div);
    println!("{} +  2: {}",int, add);
}
