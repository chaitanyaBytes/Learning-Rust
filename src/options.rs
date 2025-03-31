fn main() {
    let s = String::from("raman");
    let res = find_first_a(s);

    match res {
        None => println!("There is no a in the string"),
        Some(index) => println!("first a is found at {}", index),
    }
}

fn find_first_a(s: String) -> Option<i32> {
    for (index, character) in s.chars().enumerate() {
        if character == 'a' {
            return Some(index as i32);
        }
    }

    None
}
