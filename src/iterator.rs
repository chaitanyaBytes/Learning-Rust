// immutable iterator
fn main() {
    let vec = vec![10, 20, 30, 40];

    let iter = vec.iter();

    for val in iter {
        println!("{}", val);
    }

    println!("{:?}", vec);
}

// mutable iterator
fn main() {
    let mut vec = vec![10, 20, 30, 40];

    let iter = vec.iter_mut();

    for val in iter {
        println!("{}", val);
    }

    println!("{:?}", vec);
}

fn main() {
    let mut vec = vec![10, 20, 30, 40];

    let mut iter = vec.iter_mut();

    while let Some(val) = iter.next() {
        println!("{}", val)
    }

    println!("{:?}", vec);
}

// the above 2 iterators proviedes a way to iterate over the collection
// by BORROWING them

// into_iter
// the above 2 iterators proviedes a way to iterate over the collection
// by taking OWNERSHIP of them
fn main() {
    let vec = vec![10, 20, 30, 40];

    let mut iter = vec.into_iter();

    while let Some(val) = iter.next() {
        println!("{}", val)
    }
}
// this helps in memory mgmt
// in other iters, we have the reference and thus we have to hop to reach the address

// consuming adapter
// methods on iterators that call the "next()", under the hood
// calling them uses up the adapter
fn main() {
    let vec = vec![10, 20, 30, 40];

    let iter = vec.iter();

    // sum function takes self as an input and not &self which means it takes ownership and is not borrowed
    let total: i32 = iter.sum();
    println!("{}", total);

    // cannot use the iter anymore
    // for val in iter {
    //     println!("{}", val);
    // }
}

// iterator adapter
// dont consume the iterator
// produce a diff iterator by changing some aspects of original iterator
fn main() {
    let vec = vec![10, 20, 30, 40];

    let iter = vec.iter();

    // takes iter as input and returns a new iterator of the original vector
    // well this also moves the ownership from prev iter to itself
    let iter2 = iter.map(|x| x + 1);

    for val in iter2 {
        println!("{}", val)
    }

    let iter3 = iter.filter(|x| *x % 4 == 0);
    for val in iter3 {
        println!("{}", val)
    }
}

fn main() {
    let mut vec = vec![10, 20, 30, 40];

    let iter = vec.iter().filter(|x| *x % 4 == 0).map(|x| x + 1);

    vec = iter.collect();

    println!("{:?}", vec);
}
