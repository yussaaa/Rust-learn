// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function
struct Person {
    age: u32,
    name: String,
    favorite_color: String,
}

// impl Person {
//     fn print_person(&self) {
//         println!("Name: {}", self.name);
//         println!("Favorite Color: {}", self.favorite_color);
//     }
// }
fn print_person(person : &Person) {
        println!("Name: {}", person.name);
        println!("Favorite Color: {}", person.favorite_color);
    }

fn main() {
    let people = vec![
        Person {
            age: 10,
            name: String::from("Bob"),
            favorite_color: String::from("Red"),
        },
        Person {
            age: 20,
            name: String::from("Sally"),
            favorite_color: String::from("Blue"),
        },
        Person {
            age: 30,
            name: String::from("Joe"),
            favorite_color: "Green".to_string(),
        },
    ];

    for person in people {
        if person.age <= 10 {
            // person.print_person();
            print_person(&person);
        }
    }
}
