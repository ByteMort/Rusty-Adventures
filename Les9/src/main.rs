fn main() {
    // Tuples
    // Allows us to keep different types of data together

    let mut my_tuple:(i32, f32, bool) = (10, 3.14, false);
    println!("Second element of my_tuple: {}", my_tuple.1);

    let (a, b, c) = my_tuple;
    println!("A: {}", a);
    println!("B: {}", b);
    println!("C: {}", c);

    my_tuple.2 = true;
    println!("{}", my_tuple.2);

    let (x, y) = get_coords();
    println!("X: {}\nY: {}", x, y);

    // We can place the tuple inside the tuple
    let nested:((i32, i32), (bool, bool), bool) = ((1, 2), (true, false), true);
    println!("First Element: {:?}", nested.0);
    println!("Second Element: {:?}", nested.1);
    println!("Last Element: {}", nested.2);

    // You can move the elements with the tuple
    let mut a = 5;
    let mut b = 10;
    println!("a: {}\nb: {}", a, b);
    (a, b) = (b, a);
    println!("a: {}\nb: {}", a, b);

    // Tuple match
    let event:(&str, i32, i32) = ("click", 150, 200);
    match event{
        ("click", x, y) => println!("Mouse clicked at ({}, {})", x, y),
        ("move", x, y) => println!("Mouse moved to ({}, {})", x, y),
        _ => println!("Unknown event"),
    }
}

fn get_coords() -> (i32, i32){
    // Allows us to return more than one value with the function
    (3, 7)
}