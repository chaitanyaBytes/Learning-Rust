// borrowing means passing as reference
// using &var_name

/* Rust’s Borrowing Rules
✅ One mutable reference (&mut) at a time.
✅ Can have multiple immutable references (&), but no &mut while & exists.
✅ Mutable reference goes out of scope before a new one is created.
*/

fn main() {
    let s1: String = String::from("Hello");
    update_word(&s1);
    println!("{}", s1);
}

fn update_word(s: &String) {
    println!("{}", s);
}

// I can have as many read only borrowers as i want
fn main() {
    let s1: String = String::from("Hello");
    update_word(&s1);
    update_word(&s1);
    update_word(&s1);
    update_word(&s1);
    println!("{}", s1);
}

fn update_word(s: &String) {
    println!("{}", s);
}

// but if i want to have a borrower to mutate it, i can only have 1 of that type
// and no more read only borrowers
// using &mut var_name
// cannot do hanky panky with multple at the same time
fn main() {
    let mut s1: String = String::from("Hello");
    update_word(&mut s1);
    println!("{}", s1);
}

fn update_word(s: &mut String) {
    s.push_str("World");
    println!("{}", s);
}

// oh well, this doesn't give an error either
fn main() {
    let mut s1: String = String::from("Hello");
    update_word(&mut s1); // ✅ `update_word` takes `&mut s1`, but it is released after execution
    update_word(&mut s1);
    println!("{}", s1);
}

fn update_word(s: &mut String) {
    s.push_str(" World");
    println!("{}", s);
}
// nice
// Why This Works?
// update_word(&mut s1) takes a mutable reference, but when the function returns, that reference is dropped.
// Since update_word finishes execution, s1 is no longer borrowed.

// then waht does actually give an error???

fn main() {
    let mut s1: String = String::from("Hello");
    let s2 = &mut s1; // no error: can have  one mutable borrower
    let s3 = &mut s1; // error: cannot have more than one mutable borrowers
    let s4 = &s1; // error: cannot borrow as immutable because it is already borrowed as mutable
}

// note: ✅ Rust allows mutable borrowing as long as there are no active borrows at the same time.

// fix for above
fn main() {
    let mut s1: String = String::from("Hello");
    {
        let s2 = &mut s1; // ✅ Mutable reference inside a limited scope
        println!("{}", s2);
    } // 🔄 `s2` goes out of scope here

    update_word(&mut s1); // ✅ Now we can borrow `s1` again
    println!("{}", s1);
}

fn update_word(s: &mut String) {
    s.push_str(" World");
    println!("{}", s);
}

// trivia
// in this one try reordering the statements and you'll abserve when you get errors and when you don't
// the borrower (refernce) only exists till it has its value used someweher in case of normal scope
// in case of a function it exists till a funtion executes completely
fn main() {
    let mut s1: String = String::from("Hello");

    let s2 = &s1;
    println!("{}", s2);

    let s3 = &s1;
    println!("{}", s3);

    update_word(&mut s1);
    println!("{}", s1);
}

fn update_word(s: &mut String) {
    s.push_str(" World");
    println!("{}", s);
}
