
fn main() {
    let numbers = [1, 2, 3, 4, 5];
    let result = task_array(&numbers);
    println!("The sum of the array is: {}", result);
}

fn task_array(arr: &[i32]) -> i32 {
    let mut sum = 0;
    for &item in arr.iter() {
        sum += item;
    }
    sum
}
//calculate sum takes a reference not ownership