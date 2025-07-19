fn main() {
    // Functions
    say_hello();

    let me:String = say_my_name("Mortwain");
    println!("{}", me);

    let me2:String = say_my_name2("Mortwain");
    println!("{}", me2);

    // Anonymous Functions (Closures)
    let greet = |name:&str| -> String {
        format!("Hello, {}.", name)
    };
    println!("{}", greet("Mortwain"));

    let square = |x:i32| x*x;
    println!("3^2 = {}", square(3));
    println!("5^2 = {}", square(5));

    let result = apply(add_one, 5);
    println!("Result: {}", result);
}

fn say_hello(){
    println!("Hello World!");
}

fn say_my_name(name:&str) -> String {
    return format!("I am {}.", name);
}

fn say_my_name2(name:&str) -> String {
    format!("I am {}.", name)
}

fn add_one(x:i32) -> i32 {
    x+1
}
fn apply(f:fn(i32)->i32, val:i32) -> i32 {
    f(val)
}