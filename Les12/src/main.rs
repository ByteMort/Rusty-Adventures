#[allow(dead_code)]

#[derive(Debug, Copy, Clone)]
enum Color{
    Red,
    Green,
    Blue,
}

#[derive(Debug, Clone)]
enum Message{
    Quit,
    Move {x:i32, y:i32},
    Text(String),
    Value(i32, i32, i32)
}

impl Message{
    fn control(&self){
        match self {
            Message::Quit => println!("Left"),
            Message::Move{x, y} => println!("Moved x:{}, y:{}", x, y),
            //Message::Move{x, ..} => println!("Moved x:{}", x),
            Message::Text(s) => println!("Text is {}", s),
            Message::Value(a, b, c) => println!("Values: {}, {}, {}", a, b, c),
        }
    }
}

// Instead of null we have option enum 
/*
enum Option<T>{
    None, 
    Some(T),
}
*/

// Struct and Enum
struct Dog{name:String, age:u8}
struct Cat{name:String, color:String}

enum Animal{
    D(Dog),
    C(Cat),
}

fn main() {
    // Enums
    let color1:Color = Color::Green;
    match color1 {
        Color::Red => println!("Red"),
        Color::Green => println!("Green"),
        Color::Blue => println!("Blue"),
    };
    println!("{:?}", color1);
    println!("{:#?}", color1);

    let m1 = Message::Move{x:15, y:20};
    let m2 = Message::Quit;
    let m3 = Message::Text("Hello".to_string());
    let m4 = Message::Value(10, 12, 14);
    m1.control();
    m2.control();
    m3.control();
    m4.control();

    
    let animal1 = Animal::D(Dog{name:"Jack".to_string(), age:10});
    let _animal2 = Animal::C(Cat{name:"Meow".to_string(), color:"White".to_string()});

    match animal1 {
        Animal::D(d) => println!("DOG: name:{}, age:{}", d.name, d.age),
        Animal::C(c) => println!("CAT: name:{}, color:{}", c.name, c.color),
    }

    let c1 = Color::Red;
    println!("{:?}", c1);
    let c2 = c1;
    println!("{:?}", c2);
    println!("{:?}", c1);

    let mes1 = Message::Text("Hey".to_string());
    println!("{:?}", mes1);
    let mes2 = mes1.clone();
    println!("{:?}", mes2);
    println!("{:?}", mes1);
} 
