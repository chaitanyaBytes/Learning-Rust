use std::thread;

fn main() {
    let x = 1;

    // closure may outlive the current function, but it borrows `v`,
    // which is owned by the current function may outlive borrowed value `v`
    {
        let v = vec![1, 2, 3];
        thread::spawn(move || {
            println!("{:?}", v);
        });
    }

    // to force the closure to take ownership of `v` (and any other referenced variables), use the `move` keyword: `move `
    println!("{}", x);
}
