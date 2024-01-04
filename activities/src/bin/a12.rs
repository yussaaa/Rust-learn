// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

struct Box {
    dimensions: (u32, u32, u32),
    weight: u32,
    color: Color,
}   

enum Color {
    Red,
    Blue,
    Green,
    Yellow,
    Orange,
    Purple,
    Black,
    White,
}

impl Color {
    fn print_color(&self) -> String {
        match self {
            Color::Red => String::from("Red"),
            Color::Blue => String::from("Blue"),
            Color::Green => String::from("Green"),
            Color::Yellow => String::from("Yellow"),
            Color::Orange => String::from("Orange"),
            Color::Purple => String::from("Purple"),
            Color::Black => String::from("Black"),
            Color::White => String::from("White"),
        }
    }
}

impl Box {
    fn new(dimensions: (u32, u32, u32), weight: u32, color: Color) -> Self {
        Self {
            dimensions,
            weight,
            color,
        }
    }

    fn print(&self) {
        println!("Dimensions: {:?}", self.dimensions);
        println!("Weight: {}", self.weight);
        println!("Color: {}", self.color.print_color());
    }
}
fn main() {
    let box1 = Box::new((1, 2, 3), 4, Color::Red);
    box1.print();
}
