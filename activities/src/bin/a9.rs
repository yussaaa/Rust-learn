// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
// * Use a function that returns a tuple
// * Destructure the return value into two variables
// * Use an if..else if..else block to determine what to print

use rand::Rng;

fn coordinate() -> (i32, i32) {
    let mut rng = rand::thread_rng();
    (rng.gen_range(0..=10), rng.gen_range(0..=10))
}

fn check (x: i32, y: i32) {
    if y > 5 {
        println!("Greater than 5");
    } else if y < 5 {
        println!("Less than 5");
    } else {
        println!("Equal to 5");
    }
}
fn main() {
    let (x, y) = coordinate();
    print!("Coordinate is ({}, {})", x, y);
    check(x, y);
}
