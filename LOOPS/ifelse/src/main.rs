fn main() {
    let net_response = 200;
    // codegets input from
    let status = if net_response ==200{true} else {false};
    //
    if status {
        println!("status is successful");
    }else{
        println!("status is failure");
    
    }
    
    let mut counter :i32=0;
    loop {
        counter=counter+1;
        println!("i am at number{counter}");
    
    if counter ==1{
        println!("starting point");
        continue
    }
    
        if counter==10{
            break;
        }
        
    }
}