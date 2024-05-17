struct User {
    name: String,
    age:i32,
    id_number:i32,
}
impl User {
    pub fn walk (&self){
        println!("User {}is walking", &self.name)
    }
    pub fn eat (&self){
        println!("User {}is eating", &self.name)
    }

    pub fn is_adult (&self){
        if self.age > 10{
            return true;
        }
        return false;
    }


}

fn main(){
    let l

}
