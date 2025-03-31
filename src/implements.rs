struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn perimeter(&self) -> u32 {
        2 * (self.height + self.width)
    }
}

fn main() {
    let rect = Rect {
        height: 10,
        width: 15,
    };

    let a = rect.area();
    let p = rect.perimeter();

    println!("area of rectangle is {}", a);
    println!("perimeter of rectangle is {}", p);
}
