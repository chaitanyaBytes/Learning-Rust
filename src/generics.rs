fn main() {
    let ans1 = larger_val(1, 2);
    let ans2 = larger_val('a', 'b');

    println!("{}", ans1);
    println!("{}", ans2);
}

fn larger_val<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}
