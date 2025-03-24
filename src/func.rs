use std::io;

fn func() {
    // Input for first number
    println!("Enter 1st num:");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Invalid input");
    let a: i32 = input1.trim().parse().expect("Invalid number");

    // Input for second number
    println!("Enter 2nd num:");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Invalid input");
    let b: i32 = input2.trim().parse().expect("Invalid number");

    // Input for operator
    println!("Enter the operator (sum, sub, mul, div):");
    let mut op = String::new();
    io::stdin().read_line(&mut op).expect("Invalid operator.");

    // Call calc function
    match calc(a, b, op.trim()) {
        Some(ans) => println!("Answer is: {}", ans),
        None => println!("Invalid operation or division by zero."),
    }
}

// Function to perform calculations safely
fn calc(a: i32, b: i32, op: &str) -> Option<i32> {
    match op {
        "sum" => Some(a + b),
        "sub" => Some(a - b),
        "mul" => Some(a * b),
        "div" => {
            if b != 0 {
                Some(a / b)
            } else {
                None
            }
        } // Avoid division by zero
        _ => None,
    }
}
