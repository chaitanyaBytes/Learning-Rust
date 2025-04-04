use std::collections::HashMap;

fn main() {
    let mut map: HashMap<String, i16> = HashMap::new();

    map.insert(String::from("chaitanya"), 21);
    map.insert(String::from("Reet"), 20);

    let my_age = map.get("c5haitanya");

    match my_age {
        Some(age) => println!("age is {}", age),
        None => println!("No key exists for the given input"),
    }
}

// problem: create a hashmap from a vector of tuples as key: value pairs
use std::{collections::HashMap, vec};

fn main() {
    let vec = vec![
        (String::from("chaitanya"), 21),
        (String::from("harkirat"), 28),
        (String::from("Reet"), 20),
    ];

    let map = group_by_values(vec);

    println!("{:?}", map);
}

fn group_by_values(vec: Vec<(String, i32)>) -> HashMap<String, i32> {
    let mut map: HashMap<String, i32> = HashMap::new();

    for (k, v) in vec {
        map.insert(k, v);
    }

    return map;
}
