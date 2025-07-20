fn main() {
    // Arrays
    // Arrays collect more than 1 value of the same data type

    // Define an array
    let mut arr1:[i32; 5] = [0; 5];
    arr1[0] = 10;
    arr1[1] = 20;
    arr1[2] = 30;
    arr1[3] = 40;
    arr1[4] = 50;

    // Print elements of an array
    println!("Element 0 of arr1: {}", arr1[0]);

    println!("Array: {:?}", arr1);

    println!("Array: {:#?}", arr1);

    println!();

    for i in arr1.iter(){
        println!("{}", i);
    }

    println!();

    for (index, value) in arr1.iter().enumerate(){
        println!("Index: {}, Value: {}", index, value);
    }

    println!();

    let arr2:[&str; 4] = ["a", "b", "c", "d"];
    println!("{}", arr2.join(", "));

    println!();

    let mut arr3:[String; 5] = [
        String::new(), String::new(),
        String::new(), String::new(),
        String::new()
    ];
    for (i, val) in arr1.iter().enumerate(){
        arr3[i] = val.to_string();
    }
    println!("{:?}", arr3.join(", "));

    println!();

    // Methods
    let arr4:[i32;0] = [];

    // Lenght of array
    println!("Lenght of arr3: {}", arr3.len());

    // Is empty control
    println!("Is arr4 empty? {}", arr4.is_empty());

    // First and Last element
    println!("First Element: {}\nLast Element: {}", arr3.first().unwrap(), arr3.last().unwrap());

    // Get element
    println!("Get Index 3: {}", arr3.get(3).unwrap());

    // Contains element
    println!("Does arr3 contain 30? {}", arr3.contains(&"30".to_string()));

    // iter_mut 
    for i in arr1.iter_mut(){
        *i += 5;
    }
    println!("Arr3: {:?}", arr1);

    // as_slice
    let slice_arr1:&[i32] = arr1.as_slice();
    println!("{:?}", slice_arr1);

    // as_mut_slice
    let mut_slice_arr1:&mut [i32] = arr1.as_mut_slice();
    for i in &mut *mut_slice_arr1{
        *i -= 5;
    }
    println!("{:?}", mut_slice_arr1);

    // Reverse and sort arr3
    arr3.reverse();
    println!("{:?}", arr3);
    arr3.sort();
    println!("{:?}", arr3);

    // sort_by
    let mut arr:[i32; 5] = [1, 2, 3, 4, 5];
    arr.sort_by(|a, b| b.cmp(a));
    println!("{:?}", arr);

    // split
    let text:&str = "Hello hi welcome";
    let array_text = text.split(" ");
    for i in array_text{
        println!("{}", i)
    }

    // split_at
    let (a, b) = arr.split_at(3);
    println!("Split1: {:?}, Split2: {:?}", a, b);

    // Chunks
    for i in arr.chunks(2){
        println!("Chunk {:?}", i);
    }

    // Rotating
    arr.rotate_left(2);
    println!("{:?}", arr);
    arr.rotate_right(4);
    println!("{:?}", arr);
    arr.rotate_left(2);
    println!("{:?}", arr);

    // windows
    for i in arr.windows(2){
        println!("{:?}", i);
    }

    // find
    let found = arr.iter().find(|&x| x%2==0);
    println!("Element: {:?}", found);

    // position
    let index = arr.iter().position(|x| *x%2==0 && *x%4!=0);
    println!("Index: {:?}", index);

    // filter
    let evens:Vec<_> = arr.iter().filter(|&x| *x%2==0).collect();
    println!("Evens: {:?}", evens);

    // any
    let has_one = arr.iter().any(|&x| x==1);
    println!("Has One {}", has_one);

    // all
    let all = arr.iter().all(|&x| x%1==0);
    println!("{}", all);

    // map
    let squared = arr.map(|x| x*x);
    println!("Squared: {:?}", squared);

    // To vector
    let vec = arr.to_vec();
    println!("Vector: {:?}", vec);

    // clone
    let arr2 = arr1.clone();
    println!("Arr1 {:?}", arr1);
    println!("Arr2 {:?}", arr2);

    // copy_from_slice
    let mut dest:[i32; 3] = [0; 3];
    let src:[i32; 3] = [100, 200, 300];
    println!("Dest {:?}", dest);
    println!("Src {:?}", src);
    dest.copy_from_slice(&src);
    println!("Dest {:?}", dest);
    println!("Src {:?}", src);

    // Dimensional array
    let matrix:[[i32;2];3] = [
        [1, 2],
        [3, 4],
        [5, 6]
    ];

    println!("Matrix: {:?}", matrix);
}
