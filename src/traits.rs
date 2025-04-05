// traits are abstract classes that either have just the declaration of a function\
// or maybe default definition also that any struct can implement

trait Summary {
    fn summarise(&self) -> String {
        return String::from("Hi there from summarise");
    }
}

trait Fix {
    fn fixes(&self) -> String {
        return String::from("Hi there from fix");
    }
}

struct User {
    name: String,
    age: i16,
}

impl Summary for User {}
impl Fix for User {}

fn main() {
    let user = User {
        name: String::from("chaitanya"),
        age: 21,
    };

    println!("{} is {} years old", user.name, user.age);
    println!("{}", user.summarise());
    println!("{}", user.fixes());

    // calling the notify function
    notify(&user);

    // error: the trait bound `String: Summary` is not satisfied
    // notify(&String::from("hello"));

    notification(user);
}

// traits as parameters
fn notify(u: &impl Summary) {
    // this means that the notify function will take any struct as arguement that
    // implements the Summary trait
    // we have specified the trait bound for the input of this fuction
    // now we cannot input any struct to the  function that does not implement the Summary trait

    println!("this is from notify function: {}", u.summarise());
}
// this is one way to specify the trait bound

// the following is the better and more used way
fn notification<T: Summary + Fix>(u: T) {
    // this means that the input can be generic but it must implement Summary and fix trait
    // no other argument is allowed for this function

    println!("this is from notification: {}", u.summarise());
    println!("this is from notification: {}", u.fixes());
}
