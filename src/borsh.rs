use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, BorshDeserialize, BorshSerialize)]
struct Person {
    name: String,
    age: u8,
}

fn main() {
    let person = Person {
        name: String::from("Chaitanya"),
        age: 21,
    };

    let mut v: Vec<u8> = Vec::new();

    person.serialize(&mut v).unwrap();

    println!("{:?}", v);

    let p = Person::try_from_slice(&v);

    println!("{:?}", p);
}
