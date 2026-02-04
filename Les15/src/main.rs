
fn main() {
    // Vectors

    // Empty Vector
    let mut my_vec1: Vec<i32> = Vec::new();

    // Vector with vec!
    let mut my_vec2: Vec<f32> = vec![3.5, 2.3, 4.55, 8.33];

    // Same value x times
    let mut my_vec3: Vec<i32> = vec![1;5];

    // With Iterators
    let my_vec4: Vec<i32> = (0..=5).collect();

    // Push
    my_vec1.push(15);
    my_vec1.push(10);
    my_vec1.push(30);

    // Insert
    my_vec1.insert(2, 88);

    // Append
    let mut my_vec5: Vec<i32> = vec![99, 77, 66];
    my_vec1.append(&mut my_vec5);

    // Extend
    my_vec1.extend([111, 222, 333]);

    for e in my_vec1.iter(){
        println!("Value {}", e);
    }

    for (i, v) in my_vec2.iter().enumerate(){
        println!("Index: {}, Value: {}", i, v);
    }

    my_vec3.iter_mut().for_each(|v:&mut i32|{
        *v+=10;
    });

    my_vec3.iter().for_each( |v| {
        println!("Value {}", v);
    });

    my_vec4.iter().enumerate().for_each(|(i, v)|{
        println!("Index: {}, Value: {}", i, v);
    });
        
    for e in my_vec1.into_iter(){
        println!("Value {}", e);
    }

    // Gives error bc into_iter() take ownership
    /* 
    for e in my_vec1.iter(){
        println!("Value {}", e);
    }*/

    // Chunks(), WIndows()
    for c in my_vec3.chunks(3){
        println!("Chunk {:?}", c);
    }

    for w in my_vec3.windows(3){
        println!("Window {:?}", w);
    }

    // [], Get(), First(), Last(), Remove(), Drain()
    println!("First Element 1 : {}", my_vec2[0]);
    println!("First Element 2 : {}", my_vec2.get(0).unwrap());
    println!("First Element 3 : {}", my_vec2.first().unwrap());

    println!("Last Element 1 : {}", my_vec2[my_vec2.len()-1]);
    my_vec2.remove(my_vec2.len()-1);
    println!("Last Element 2 : {}", my_vec2.get(my_vec2.len()-1).unwrap());
    println!("Last Element 3 : {}", my_vec2.last().unwrap());

    my_vec3.drain(1..4);
    println!("Vec3 {:?}", my_vec3);

    // With_Capacity()
    let mut my_vec6: Vec<i32> = Vec::with_capacity(7);
    println!("Capacity of Vec6: {}", my_vec6.capacity());

    for i in 1..=6{
        my_vec6.push(i);
    }
    my_vec6.iter().for_each(|v| println!("Value: {}", v));

    // Truncate()
    my_vec6.truncate(3);
    my_vec6.iter().for_each(|v| println!("Value: {}", v));

    // Reverse(), Sort(), Sort_by()
    my_vec6.reverse();
    println!("Vec6: {:?}", my_vec6);
    my_vec6.sort();
    println!("Vec6: {:?}", my_vec6);
    my_vec6.sort_by(|a, b| b.cmp(a));
    println!("Vec6: {:?}", my_vec6);

    // Swap()
    my_vec6.swap(0, 2);
    println!("Vec6: {:?}", my_vec6);

    // Swap_remove()
    my_vec6.swap_remove(1);
    println!("Vec6: {:?}", my_vec6);
    
    // Pop() 
    let last_deleted_element: i32 = my_vec6.pop().unwrap();
    println!("Last deleted element: {}", last_deleted_element);
    println!("Vec6: {:?}", my_vec6);

    // Retain()
    let mut my_vec7: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    my_vec7.retain(|v| v%2==0);
    println!("Vec7: {:?}", my_vec7);

    // There is no distinct method in Rust
    // So we use sort() and dedup()
    let mut my_vec8: Vec<i32> = vec![1, 2, 3, 1, 2, 4, 6, 3];
    println!("Vec8: {:?}", my_vec8);
    my_vec8.sort();
    my_vec8.dedup();
    println!("Vec8: {:?}", my_vec8);

    // Contains()
    println!("Vec8 Contains 5: {}", my_vec8.contains(&5));
    println!("Vec8 Contains 6: {}", my_vec8.contains(&6));

    // Position()
    println!("Position of 6 in Vec8: {}", my_vec8.iter().position(|v| *v==6).unwrap());

    // Resize()
    my_vec8.resize(3, 0);
    println!("Vec8: {:?}", my_vec8);
    my_vec8.resize(6, 33);
    println!("Vec8: {:?}", my_vec8);

    // Clone()
    let my_vec9: Vec<i32> = my_vec8.clone();
    println!("Vec9: {:?}", my_vec9);

    // Split_at()
    let (left, right) = my_vec9.split_at(2);
    println!("Left: {:?}", left);
    println!("Right: {:?}", right);

    // Binary_search()
    println!("Find 33: {}", my_vec9.binary_search(&33).unwrap());
}
