struct Person{
    name: String,
    age: u32,
    // struct in struct
    address: Address,
}

struct Address{
    city: String,
    zip: u16,
}

// implement a method
impl Person{
    fn say_hello(&self){
        println!("Hello, my name is {}", self.name);
    }

    // method with parameters
    fn have_birthday(&mut self){
        self.age += 1;
    }
}

// Tuple struct
struct Color(u8, u8, u8);

// Struct clone
#[derive(Clone)]
// Struct debug
#[derive(Debug)]
struct P{
    name: String,
    age: u8,
}

fn main() {
    // Struct
    // In Rust, a struct is a user-defined data type
    // that groups together multiple related values (fields) under a single name.

    let person1 = Person { 
        name: String::from("Mortwain"),
        age: 18,
        address: Address {
            city: "Diest".to_string(),
            zip: 3290
        }
    };

    println!("Name of person1: {}", person1.name);
    println!("Age of person1: {}", person1.age);
    println!("City of person1: {}", person1.address.city);
    println!("Zip Code of person1: {}", person1.address.zip);

    person1.say_hello();

    println!();

    // Copying elements
    let mut person2 = Person {
        name: String::from("Ali"),
        ..person1
    };

    println!("Name of person2: {}", person2.name);
    println!("Age of person2: {}", person2.age);

    person2.say_hello();
    person2.have_birthday();

    println!("Age of person2: {}", person2.age);

    println!();

    let red:Color = Color(255, 0, 0);
    println!("Color: {} {} {}", red.0, red.1, red.2);

    println!();

    let p1 = P {
        name: String::from("Talon"),
        age: 18,
    };

    let p2 = P {
        age: 22,
        ..p1.clone()
    };

    println!("Name of p1 {}", p1.name);
    println!("Age of p1 {}", p1.age);
    println!("Name of p2 {}", p2.name);
    println!("Age of p2 {}", p2.age);

    println!("{:?}", p1);
    println!("{:#?}", p1);

    read_person(&person2);
    println!("Name of person2: {}", person2.name);
}


fn read_person(p:&Person){
    println!("Name: {}\nAge: {}\nCity: {}\nZip: {}", p.name, p.age, p.address.city, p.address.zip);
}