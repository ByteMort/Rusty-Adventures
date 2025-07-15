fn main() {
    // Variables
    // We can create variables with the keywords let and const
    // The value of const variables cannot be changed
    // If we want to change the value of let variables then we use keyword mut
    // If we aren't using the variable then we start the name with an underscore
    let mut _a:i8 = 5;
    const _PI:f32= 3.451;
    _a = 15;

    // Data Types
    // Scalar Types: Integers, Floating-point numbers, booleans and characters

    // Integers can be : 8, 16, 32, 64, 128 bit or arch
    // We have unsigned and signed integers
    // Signed: positive and negative
    // Unsigned: positive
    let _x:u8 = 15;
    let mut _y:i8 = -8;
    _y = 5;

    // Floating-point numbers  can be: 32 or 64 bit
    let _a:f32 = 15.32;

    // Booleans can be: true or false
    let _true_b:bool = true;
    let _false_b:bool = false;

    // Characters can be: one single character
    let _char_b:char = 'b';  

    // Compound Types: Arrays or Tuples

    // Arrays: contain multiple values of the same data type
    let _array1:[i32; 5] = [1, 2, 3, 4, 5]; 
    let _array2:[i32; 5] = [3;5]; // This is the same as this: let _array2:[i32; 5] = [3, 3, 3, 3, 3];

    // Tuples: contain multiple values of the different data types
    let _tuple1:(i32, bool, char, f32) = (500, false, 'a', 5.5);

    // Strings: contain more than one character
    let _string1:String = String::from("Hello Guys");
    
    // &Str: contain more than one character
    let _str1:&str = "Hello World";
}  
