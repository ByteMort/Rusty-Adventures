fn main() {
    // Conditions
    let age:i32 = 18;
    let category:&str;

    // if, else if, else
    if age <= 12{
        category = "Child";
    }else if age <= 17{
        category = "Teen";
    }else if age <= 64{
        category = "Adult";
    }else{
        category = "Senior";
    }

    println!("Category: {}", category);

    // match
    let age2:i32 = 15;
    let category:&str = match age2 {
        0..=12  => "Child",
        13..=17 => "Teen",
        18..=64 => "Adult",
        _ => "Senior",
    };
    println!("Category: {}", category);
}
