use std::io;

fn main() {
    println!("Enter an integer:");

    
    let mut a = String::new();

  
    io::stdin().read_line(&mut a)
        .expect("Failed to read line");

    
    let a: i32 = a.trim().parse()
        .expect("Please enter a valid integer");

 
    let mut counter = 0;

    
    println!("Numbers from 0 to {}:", a);
    loop {
        if counter > a {
            break;
        }
        println!("{}", counter);
        counter += 1;
    }
}
