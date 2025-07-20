fn main() {
    // Loops
    
    // loop
    let mut i:i32 = 0;
    loop{
        println!("Loop i: {}", i);
        i+=1;
        if i == 5{
            break;
        }
    }

    println!();

    // while
    let mut i:i32 = 0;
    while i < 5{
        println!("While i: {}", i);
        i+=1;
    }

    println!();

    // for
    for i in 0..5{
        println!("For i: {}", i);
    }
}
