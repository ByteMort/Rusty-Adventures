fn main() {
    // Integer Methods

    let a:i32 = 15;  
    // Positive and negative control
    println!("{}", a.is_positive());
    println!("{}", a.is_negative());


    // From string to int
    let b:i32 = i32::from_str_radix("00001001", 2).unwrap();
    println!("{}", b);

    let c:i32 = i32::from_str_radix("17", 8).unwrap();
    println!("{}", c);

    let d:i32 = i32::from_str_radix("123", 10).unwrap();
    println!("{}", d);

    let e:i32 = i32::from_str_radix("A", 16).unwrap();
    println!("{}", e);

    let f:i32 = i32::from_str_radix("Z1", 36).unwrap();
    println!("{}", f);

    let g:String  = "42".to_string();
    let h:i32 = g.parse::<i32>().unwrap();
    println!("{}", h);

    let k:&str = "0101";
    let m:i32 = k.parse().unwrap();
    println!("{}", m);

    let n:f32 = 15.7;
    let z:i32 = n as i32;
    println!("{}", z);


    // Math methods
    println!("Minimum of i32: {}", i32::MIN);
    println!("Maximum of i32: {}", i32::MAX);

    let a:i16 = (-5_i16).abs();
    println!("{}", a);

    // Returns None if the absolute value would overflow; otherwise returns Some(abs value)
    let b:i16 = -10;
    let c:i16 = b.checked_abs().unwrap();
    println!("{}", c);

    // If the absolute value overflows, it does not ‘abs’, it returns the value in the beginning
    let d:i8 = i8::MIN;
    let e:i8 = d.wrapping_abs();
    println!("{}", e);

    // If the absolute value overflows, it returns the max value of that integer type
    let f:i8 = i8::MIN;
    let g:i8 = f.saturating_abs();
    println!("{}", g);

    // Returns the sign of the number
    let a:i8 = -5;
    println!("{}", a.signum());

    // Value Limitation
    let x:i16 = 120;
    let limited_x:i16 = x.clamp(0, 100);
    println!("{}", limited_x);

    // Euclidean division in negative numbers
    let a:i8 = -9;
    let b:i8 = 4;
    println!("{}", (a/b));
    println!("{}", a.div_euclid(b)); 
    println!("{}", a.rem_euclid(b));

    // Exponentiation of a number
    let x:i8 = 2;
    let y:f64 = 3.5;
    println!("{}", x.pow(3));
    println!("{}", y.powi(3));

    // Binary
    let a:u8 = 0b1011;
    println!("{}", a.count_ones());
    println!("{}", a.count_zeros());

    // SQRT
    println!("{}", i32::isqrt(16_i32));
    println!("{}", 16_i32.isqrt());

}
