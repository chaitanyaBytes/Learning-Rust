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

    let slice = &s[0..ind];
    println!("{}", slice);
    println!("{}", s);
}
