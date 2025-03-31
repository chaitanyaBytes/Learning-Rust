use std::fs;

// error in rust is handled by declaring a result enum
// this one handles the error gracefully
fn main() {
    let res = fs::read_to_string("example.txt");

    match res {
        Ok(content) => println!("contents are: {}", content),
        Err(err) => println!("There is an error: {}", err),
    }
}

use std::fs;

// this example does not handle errors and program crashes
fn main() {
    let res = read_from_file("file.txt".to_string());
    println!("{}", res);
}

fn read_from_file(file_path: String) -> String {
    let res = fs::read_to_string(file_path);
    return res.unwrap();
}

use std::fs;

// 3rd category
fn main() {
    let res = read_from_file("file.txt".to_string());
    match res {
        Ok(content) => println!("{}", content),
        Err(e) => println!("Error: {}", e),
    }
}

fn read_from_file(file_path: String) -> Result<String, String> {
    let res = fs::read_to_string(file_path);

    if let Ok(content) = res {
        Ok(content)
    } else {
        Err("Erro reading the file".to_string())
    }
}
