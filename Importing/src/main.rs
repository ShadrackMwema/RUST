#[path = "directory_module/module.rs"]
mod directory_module;

mod file_module;

fn main() {
    let slice = [1, 9, 3, 4, 7];

    let second_item = file_module::retrieve(&slice);

    println!("The second item is: {}", second_item);

    // Call directory_module::print() and store its result in a variable
    let dir_result = directory_module::print();

    // Now, print the result stored in the variable
    println!("FROM DIRECTORY MODULE: {}", dir_result);
}
