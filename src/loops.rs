fn loops() {
    // loops
    for i in 0..10 {
        println!("{} ", i);
    }

    let sentence: String = String::from("hello world!");
    let first_word: String = get_first_word(sentence);

    println!("first word is: {}", first_word);
}

fn get_first_word(sentence: String) -> String {
    let mut str: String = String::from("");

    for char in sentence.chars() {
        str.push_str(char.to_string().as_str());

        if char == ' ' {
            break;
        }
    }

    return str;
}
