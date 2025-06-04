// Rust can’t guarantee that the reference you return (&str) will live as long as both input references.
// If one of the inputs dies early (goes out of scope),
// your return value might point to invalid memory — and Rust hates that.

// this one is the error code
fn longest(s1: &str, s2: &str) -> &str {
    // generic lifetime annotation
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

fn main() {
    let long_str;
    let s1 = String::from("hello");
    {
        let s2 = String::from("Weorld");
        long_str = longest(&s1, &s2);
    }

    println!("longest string is {}", long_str);
}

// this code is fine
// "Both s1 and s2 must live at least as long as 'a, and the return value will live as long as 'a too."
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    // generic lifetime annotation
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
    long_str = longest(&s1, &s2);

    println!("longest string is {}", long_str);
}
// “Trust me, these borrows all live at least as long as 'a, so returning one is safe.”

// struct with lifetiemes
// "name is a reference that must live at least as long as lifetime 'a"
struct User<'a> {
    name: &'a str,
    age: i8,
}

fn main() {
    let name = String::from("chaitanya");
    let user = User {
        name: &name,
        age: 21,
    };
    // if name goes out scope, user should also go out of scope

    println!("name is {} and age is {}", user.name, user.age);
}

// lifetimes with generics
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
    {
        let s2 = String::from("Weorld");
        long_str = longest(&s1, &s2, String::from("Hello world"));
        println!("longest string is {}", long_str);
    }
}

// The returned string slice will live as long as both s1 and s2 — or more specifically, as long as the shortest of their lifetimes.
// Lifetimes don’t change how long something actually lives — they just describe it.
// You're right: it's like saying "this return reference is tied to the lifetime of the inputs."
