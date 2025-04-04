fn main() {
    let mut vec = vec![10, 20, 30, 40];

    let iter = vec.iter().filter(|x| *x % 4 == 0).map(|x| x + 1);

    vec = iter.collect();

    println!("{:?}", vec);
}
