
//Managing books Each book has a name , status , Category
//Using structs...implement

struct Book {
    name : String,
    Status : bool,
    Category : String,

}

impl Book {
    pub fn Avail (&self) -> bool {
      return self.Status;
    }


}
fn main(){
    let RUST = Book {
        name: String::from("RUST"),
        Status: false,
        Category: String::from("RUST"),

    };
    println!("Status{}",RUST.Avail())
}