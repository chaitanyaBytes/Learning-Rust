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
