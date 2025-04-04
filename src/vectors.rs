fn main() {
    // initialsing usign struct
    let mut vec1: Vec<u8> = Vec::new();
    vec1.push(1);
    vec1.push(2);

    // initialising using macro
    let vec2: Vec<u8> = vec![1, 2, 3];

    println!("{:?}", vec1);
    println!("{:?}", vec2);
}

// approach 1
// creating a new vector in the function and passing the ownership and retruning it
fn main() {
    let mut vec: Vec<u8> = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    vec.push(4);
    vec.push(5);

    //{:?} this is a debug trait will learn later

    println!("{:?}", filter_even(vec));
}

fn filter_even(vect: Vec<u8>) -> Vec<u8> {
    let mut ans: Vec<u8> = Vec::new();

    for ele in vect {
        if ele % 2 == 0 {
            ans.push(ele);
        }
    }

    return ans;
}

// approch 2:
// passing as mutable reference and doing the filtering in place
fn main() {
    let mut vec: Vec<u8> = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    vec.push(4);
    vec.push(5);

    //{:?} this is a debug trait will learn later

    filter_even(&mut vec);
    println!("{:?}", vec);
}

fn filter_even(vec: &mut Vec<u8>) {
    let mut i = 0;

    while i < vec.len() {
        if vec[i] % 2 == 1 {
            vec.remove(i);
        }
        i = i + 1;
    }
}
