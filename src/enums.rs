// this is an example of a simple enum
enum Direction {
    North,
    South,
    East,
    West,
}

fn main() {
    let dir = Direction::East;

    move_around(dir);
}

fn move_around(dir: Direction) {
    // some function
}

use std::f64::consts::PI;

// Define an enum called Shape with each enum having some associated data as well
enum Shape {
    Circle(f64),         // Variant with associated data (radius)
    Square(f64),         // Variant with associated data (side length)
    Rectangle(f64, f64), // Variant with associated data (width, height)
}

// Function to calculate area based on the shape
fn calculate_area(shape: Shape) -> f64 {
    // calculates the area of the shape

    // pattern mathcing
    match shape {
        Shape::Circle(radius) => PI * radius * radius,
        Shape::Square(side) => side * side,
        Shape::Rectangle(width, height) => width * height,
    }
}

fn main() {
    // Create instances of different shapes
    let circle = Shape::Circle(5.0);
    let square = Shape::Square(4.0);
    let rectangle = Shape::Rectangle(3.0, 6.0);

    calculate_area(circle);
    calculate_area(square);
    calculate_area(rectangle);
}
