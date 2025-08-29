fn main() {
    // Error propagation

    match do_something(){
        Ok(v) => println!("It works {}", v),
        Err(e) => println!("Something went wrong! {}", e)
    }
    
}

fn might_fail() -> Result<u8, String>{
    let a:u8 = 10;
    let b:u8 = 0;

    if b==0{
        Err("Division by zero".to_string())
    }else{
        Ok(a/b)
    }
}

fn do_something() -> Result<u8, String> {
    let value:u8 = might_fail()?; // If there is an error, it automatically returns an error.
    //  r#try!(might_fail()); // same as ?
    //  might_fail().unwrap(); // If there is an error, it panics and stops
    //  might_fail().expect("HMM!"); // If there is an error, it panics, displays the message, and stops.
    Ok(value)
}

