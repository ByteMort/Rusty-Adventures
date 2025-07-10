use std::ops::Not;

fn main() {
    // Boolean Methods
    let t:bool = true;
    let f:bool = false;

    println!("{}", t);
    println!("{}", t.not());

    let x = t.then(|| 42);
    println!("{:?}", x);

    let y = f.then(|| 42);
    println!("{:?}", y);

    let x = t.then_some("Correct!");
    println!("{:?}", x);

    // Equals
    println!("{}", t.eq(&f));
    
    // Not Equals
    println!("{}", t.ne(&f));


    // Bitwise Operators
    println!("AND: {}", t & f);
    println!("OR: {}", t | f);
    println!("XOR: {}", t ^ f);


    // Character Methods
    let c:char = 'a';
    let d:char = '9';
    let e:char = ' ';
    let f:char = 'O';

    println!("Unicode: {}", c as u32);
    println!("Is Alphabetic? {}", c.is_alphabetic());
    println!("Is Digit? {}", d.is_digit(10));
    println!("Is Numeric {}", d.is_numeric());
    println!("Is Whitespace? {}", e.is_whitespace());
    println!("Is Uppercase? {}", f.is_uppercase());
    println!("Is Lowercase? {}", f.is_lowercase());
    println!("Lowercase: {}", f.to_lowercase());
    println!("Uppercase: {}", f.to_uppercase());
    println!("Is Alphanumeric? {}", c.is_alphanumeric());
    println!("Is ASCII Alphanumeric? {}", c.is_ascii_alphanumeric());
    println!("Is ASCII? {}", c.is_ascii());
    println!("Is ASCII Uppercase? {}", c.is_ascii_uppercase());
    println!("Is ASCII Lowercase? {}", c.is_ascii_lowercase());
    
    let m:String = c.to_string(); 
    println!("{}", m);

    println!("Lowercase: {}", f.to_lowercase().collect::<String>());
    let k:char = '\t';
    println!("Is Control? {}", k.is_control());
}
