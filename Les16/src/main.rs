use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};


fn main() {
    // Collections

    // Vec<T> => Dynamic array
    let mut my_vec:Vec<i32> = Vec::new();
    my_vec.push(3);
    println!("Vec: {:?}", my_vec);

    // HashMap<K, V> => Key And Value
    let mut my_map:HashMap<String, i32> = HashMap::new();
    my_map.insert(String::from("Ali"), 21);
    println!("HashMap: {:?}", my_map);

    println!("Contains Ali?: {}", my_map.contains_key("Ali"));
    my_map.entry("Bob".to_string()).or_insert(55);
    println!("HashMap: {:?}", my_map);

    // HashSet<T> => Unique Elements
    let mut my_set:HashSet<i32> = HashSet::new();
    my_set.insert(1);
    my_set.insert(1);
    println!("HashSet: {:?}", my_set);

    // U can use intersection(),  difference(), symmetric_difference() with same way
    let my_set2:HashSet<i32> = HashSet::from([3,4,1,6]);
    println!("HashSet2: {:?}", my_set2);
    let my_set1_and2:HashSet<i32> = my_set.union(&my_set2).cloned().collect();
    println!("HashSet 1and2: {:?}", my_set1_and2);

    // BTreeMap<K, V> => Sequential Key
    let mut my_btree_map:BTreeMap<String, i32> = BTreeMap::new();
    my_btree_map.insert("Bob".to_string(), 21);
    my_btree_map.insert("Ali".to_string(), 21);
    my_btree_map.insert("Zed".to_string(), 20);
    my_btree_map.insert("Diana".to_string(), 19);
    println!("BTreeMap: {:?}", my_btree_map);

    my_btree_map.range("A".to_string().."Zzz".to_string()).for_each(|(k, v)| println!("Key:{}, Value:{}", k, v));

    // BTreeSet<T> => Sequential unique elements
    let mut my_btree_set:BTreeSet<i32> = BTreeSet::new();
    my_btree_set.insert(3);
    my_btree_set.insert(1);
    my_btree_set.insert(1);
    println!("BTreeSet: {:?}", my_btree_set);

    my_btree_set.range(0..).for_each(|v| println!("Value: {}", v));
    // U can use union(), intersection(), difference(), symmetric_difference()
}
