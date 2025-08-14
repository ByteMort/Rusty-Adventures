fn main() {
    // Ownership
    /*
        Each value in Rust has a single owner.
        When the owner goes out of scope, the value is automatically dropped.
        When ownership is transferred(moved) to another variable, the previous owner becomes invalid.
    */

    let s1:String = String::from("Hello");
    println!("S1: {}", s1);
    let s2:String = s1;
    // println!("{}", s1);
    println!("S2: {}", s2);

    // Simple types (such as i32, bool, char, float, &str) have the Copy trait and are copyable.
    // Data types stored in the heap (String, Vec, Box, etc.) are moved.
    let x:u8 = 5;
    let y:u8 = x;
    println!("X: {}", x);
    println!("Y: {}", y);

    let s:String = String::from("World");
    print_it(s);
    // println!("S: {}", s);

    let b:String = String::from("Hello");
    // We can use clone() if we dont want to lose owner
    print_it(b.clone());
    println!("B: {}", b);

    let n:u8 = 8;
    print_it_num(n);
    println!("N: {}", n);

    // Returning to get back ownership
    let s1:String = String::from("S1");
    let s2:String = pr_s(s1);
    println!("S2: {}", s2);

    // Move
    let number_str:String = String::from("5");
    let move_f = move || {
        println!("U8: {}", number_str);
    };
    move_f();
    // println!("Number: {}", number_str);


    // Borrowing
    // Being able to use the contents of a variable without owning it.

    /*
        You can have many immutable borrows at the same time.
        You can have only one mutable borrow at a time.
        You can't have mutable and immutable borrows at the same time.
        The borrowed data cannot be modified by the original owner until the borrow ends.
    */

    let name = String::from("Mortwain");
    print_name(&name);
    println!("Name: {}", name);

    let mut msg = String::from("Hello");
    update_msg(&mut msg);
    println!("Updated: {}", msg);
    // let msg2 = &mut msg;
    // let msg3 = &mut msg;
    // println!("{}, {}", msg2, msg3);
    
    let _s = String::from("HI");
    // let r1 = &_s;
    // let r2 = &mut _s;
    // println!("{}, {}", r1, r2);

    let mut data = String::from("Rust");
    let r = &data;
    println!("{}", r);
    data.push_str(" Lang");
    println!("{}", data);
    // println!("{}", r);
}

fn print_it(w:String){
    println!("W: {}", w);
}

fn print_it_num(num:u8){
    println!("Num: {}", num);
}

fn pr_s(s:String) -> String{
    println!("S: {}", s);
    s
}

fn print_name(n: &String){
    println!("Name is: {}", n);
}

fn update_msg(text: &mut String){
    text.push_str(", World");
}