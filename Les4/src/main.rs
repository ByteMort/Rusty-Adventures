fn main() {
    // String and &str
    // Differences
    
    // String:  owned string type, mutable
    // &str:    borrowed string type, immutable
    // We will talk about ownership and borrowing in our next lessons. 

    // String methods
    let mut s1:String = String::from("Hello");
    println!("{:}", s1);

    // It adds string 
    s1.push_str(" World!");
    println!("{:}", s1);

    // It adds char
    s1.push(' ');
    println!("{:}", s1);

    // It inserts a char to a specific section
    s1.insert(3, 'b');
    println!("{:}", s1);

    // It inserts a string to a specific section
    s1.insert_str(4, "ro");
    println!("{:}", s1);

    // It removes a char 
    s1.remove(3);
    println!("{:}", s1);

    // It removes from index to index
    s1.drain(3..5);
    println!("{:}", s1);

    // It replaces strings
    let new_s1 = s1.replace("l", "hey");
    println!("{:}", s1);
    println!("{:}", new_s1);

    // It deletes whitespaces from beginning and end
    let s2:String = String::from("   s   ");
    println!("{:}", s2);
    let new_s2 = s2.trim();
    println!("{:}", new_s2);

    // To Uppercase and Lowercase
    println!("{:}", s1.to_uppercase());
    println!("{:}", s1.to_lowercase());

    // It splits string
    let splited_s1= s1.split('l');
    for s in splited_s1{
        println!("{:}", s);
    }

    // It gives length of string
    println!("{:}", s1.len());


    // &str methods
    let s2:&str = "Welcome! Heycome";

    // It gives length(byte) of &str
    println!("{:}", s2.len());

    // It checks that &str is empty
    println!("{:}", s2.is_empty());

    // It checks that &str contains something
    println!("{:}", s2.contains("c"));

    // It checks that &str starts\ends with something
    println!("Starts-with(W): {:}", s2.starts_with("W"));
    println!("Ends-with(!): {:}", s2.ends_with("!"));

    // It deletes whitespaces from beginning and end
    let s3:&str = "   ss   ";
    println!("{:}", s3);
    let new_s3 = s3.trim();
    println!("{:}", new_s3);

    // To uppercase and lowercase
    println!("{:}", s2.to_uppercase());
    println!("{:}", s2.to_lowercase());

    // It finds something from &str
    println!("{:}", s2.find("co").unwrap());

    // It splits string
    for w in s2.split("c"){
        println!("{:}", w);
    }
    for w in s2.split_whitespace(){
        println!("{:}", w);
    }

    // From string to char
    for c in s2.chars(){
        println!("{:}", c);
    }

    // It replaces &str
    let new_s2 = s2.replace("come", "D");
    println!("{:}", new_s2);

}
