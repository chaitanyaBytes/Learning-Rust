fn main() {
    let mut vec1: Vec<u8> = Vec::new();
    vec1.push(1);
    vec1.push(2);

    let vec2: Vec<u8> = vec![1, 2, 3];

    println!("{:?}", vec1);
    println!("{:?}", vec2);
}
