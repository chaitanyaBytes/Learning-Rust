fn main() {
    let mut s = String::from("chaitanya");
    println!("{}", s);
    // push more charcters into the string
    s.push_str(" Gupta");
    println!("{}", s);

    // remove charcaters from string
    s.replace_range(5..s.len(), "u");
    println!("{}", s);
}

// simple slicing
fn main() {
    let s = String::from("Chaitanya gupta");
    println!("{}", s);

    let mut ind = 0;
    for i in s.chars() {
        if i == ' ' {
            break;
        }
        ind += 1;
    }

    let slice = &[0..ind];
    println!("{}", slice);
    println!("{}", s);

    // if you try to mutate or clear s before the scope of slice dies, it will throw an error
    // because you cant borrow a mutable refernce if an imutable reference already exists
}

// function to returnn only the first word of the input string
fn main() {
    let s = String::from("Chaitanya gupta");
    println!("{}", s);

    let ans1 = first_word_1(&s); // pass as reference!
    let ans2 = first_word_2(&s); // pass as reference!

    match ans1 {
        Some(val) => println!("{}", val),
        None => println!("String is empty"),
    }

    match ans2 {
        Some(val) => println!("{}", val),
        None => println!("String is empty"),
    }
}

fn first_word_1(s: &str) -> Option<String> {
    // &str instead of String
    let mut iter = s.split_ascii_whitespace();

    match iter.next() {
        Some(val) => Some(val.to_string()),
        None => None,
    }
}

fn first_word_2(s: &str) -> Option<String> {
    // next() gives an Option<&str>
    //.map(...) transforms the inner value only if it’s Some(...)
    s.split_ascii_whitespace().next().map(|val| val.to_string())
}
