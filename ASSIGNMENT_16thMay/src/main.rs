
#[path = "directory_module/module.rs"]
mod directory_module;

mod file_module;

fn main() {
    let slice = [1, 9, 3, 4,7];


    let second_item = file_module::retrieve(&slice);

    println!("The second item is: {}", second_item);

   directory_module::print();


}
