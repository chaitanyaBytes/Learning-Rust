use std::fmt::Display;
fn longest<'a, T>(s1: &'a str, s2: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    // generic lifetime annotation
    println!("announcment! {ann}");
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

fn main() {
    let long_str;
    let s1 = String::from("hello");
    let s2 = String::from("Weorld");
    long_str = longest(&s1, &s2, String::from("Hello world"));

    println!("longest string is {}", long_str);
}
