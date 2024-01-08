// Topic: Result
//
// Requirements:
// * Create an structure named `Adult` that represents a person aged 21 or older:
//   * The structure must contain the person's name and age
//   * Implement Debug print functionality using `derive`
// * Implement a `new` function for the `Adult` structure that returns a Result:
//   * The Ok variant should contain the initialized structure, but only
//     if the person is aged 21 or older
//   * The Err variant should contain a String (or &str) that explains why
//     the structure could not be created
// * Instantiate two `Adult` structures:
//   * One should be aged under 21
//   * One should be 21 or over
// * Use `match` to print out a message for each `Adult`:
//   * For the Ok variant, print any message you want
//   * For the Err variant, print out the error message

#[derive(Debug)]
struct Adult {
    name: String,
    age: u8,
}
// fn new(name: String, age: u8) -> Result<Adult, String> {
//     if age >= 21 {
//         Ok(Adult { name, age })
//     } else {
//         Err("Too young".to_owned())
//     }
// }
impl Adult {
    fn new(name: &str, age: u8) -> Result<Self, &str> {
        if age >= 21 {
            Ok(Self { name: name.to_string(), age })
        } else {
            Err("Too young")
        }
    }
}

fn print_adult(person: Result<Adult, &str>) {
    match person {
        Ok(person) => println!("{:} is {:}", person.name, person.age),
        Err(err) => println!("{}", err),
    }
}

fn main() {
    // let person1 = new("John".to_owned(), 20);
    // let person2 = new("Jane".to_owned(), 21);
    let p1 = Adult::new("John", 20);
    let p2 = Adult::new("Jane", 21);

    print_adult(p1);
    print_adult(p2);
}
