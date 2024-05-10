fn main() {
    
    let v = &["apples", "cake", "coffee"];

    for text in v {
    println!("I like {}.", text);
    }
        
    let array = [5,10,15,20,50];
    for count in array{
     if count > 10 {
         println!("{} is greater than 10", count);
     } else {
         println!("{} is less than 10", count);
     }
    }
 
 
     
 }
 
 