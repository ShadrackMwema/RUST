fn main() {   
    let number_1: i32 = 11;
    let number_2: i32 = 20;
    let result = sum(number_1, number_2);
    let total_ages = sum_2(20, 15);
    println!("Sum is {}", result);
    println!("Ages is {}", total_ages);
}

fn sum(number_1: i32, number_2: i32) -> i32 {
    let number_3: i32 = number_1 + number_2;
    number_3
}
//function2
fn sum_2(age1: i32, age2: i32) -> i32 {
    let total_ages: i32 = age1 + age2;
    total_ages
}
