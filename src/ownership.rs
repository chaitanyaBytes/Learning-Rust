fn main() {
    let s1: String = String::from("Hello");
    update_word(s1);
}

fn update_word(s: String) {
    println!("{}", s);
}

// this code will produce an error because the ownwership of s1 is transfered to
// the function variable and thus the var s1 does not exist anymore
fn main() {
    let s1: String = String::from("Hello");
    update_word(s1);
    println!("{}", s1);
}

fn update_word(s: String) {
    println!("{}", s);
}

// this code helps to transfer the ownership back to the same var by making it mutable
fn main() {
    let mut s1: String = String::from("Hello");
    s1 = update_word(s1);
    println!("{}", s1);
}

fn update_word(s: String) -> String {
    println!("{}", s);
    return s;
}

// another way to get bback ownership but in some other variable
fn main() {
    let s1: String = String::from("Hello");
    let s2: String = update_word(s1);
    println!("{}", s2);
}

fn update_word(s: String) -> String {
    println!("{}", s);
    return s;
}
