

fn main() {
    let mut numbers = [0; 50];

    for i in 0..50 {
        numbers[i] = 50 - i;
    }

    for num in numbers.iter() {
        println!("{}", num);
    }
}
