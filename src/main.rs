use std::fs;

// error in rust is handled by declaring a result enum
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
