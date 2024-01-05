// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

///
struct Student {
    name: String,
    locker: Option<i32>,
}
/// 
fn main() {
    let students = vec![
        Student {
            name: "John Doe".to_string(),
            locker: Some(10),
        },
        Student {
            name: "Jane Doe".to_owned(),
            locker: None,
        },
        Student {
            name: String::from("Sally Doe"),
            locker: Some(20),
        },
    ];

    for s in students {
        match s.locker {
            Some(locker) => println!("{} is in locker {}", s.name, locker),
            None => println!("{} does not have a locker", s.name),
        }
    }
}
