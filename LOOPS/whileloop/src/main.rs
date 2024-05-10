fn main() {
    // Define an array of integers
    let array: [i32; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
    // Initialize the index to the last element of the array
    let mut index = 0;

    // Display a message indicating that even numbers will be displayed
    println!("Displaying even numbers from the array:");
    
    // Loop through the array elements  using a while loop
    while index >= 0 {
        // Check if the current number is even
        if array[index as usize] % 2 == 0 { // Check if the number is even
            // If the number is even, print its index and value
            println!("Even number found at index {}: {}", index, array[index as usize]);
        }
        // increnet the index 
        index += 1;
    }
    
    // Display a message indicating the end of the loop
    println!("LIFTOFF!!!");
}
