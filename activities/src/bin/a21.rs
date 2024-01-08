// Topic: Map combinator
//
// Requirements:
// * Given a user name, create and print out a User struct if the user exists
//
// Notes:
// * Use the existing find_user function to locate a user
// * Use the map function to create the User
// * Print out the User struct if found, or a "not found" message if not

#[derive(Debug)]
struct User {
    user_id: i32,
    name: String,
}

/// Locates a user id based on the name.
fn find_user(name: &str) -> Option<i32> {
    let name = name.to_lowercase();
    match name.as_str() {
        "sam" => Some(1),
        "matt" => Some(5),
        "katie" => Some(9),
        _ => None,
    }
}

fn main() {
    // let user_id = find_user("Sam").map(|id| id);
    // println!("{:?}", user_id);
    // let user_id = find_user("mAtt").map(|id| id);
    // println!("{:?}", user_id);
    // let user_id = find_user("kaTie").map(|id| id);
    // println!("{:?}", user_id);
    // let user_id = find_user("joe").map(|id| id);
    // println!("{:?}", user_id);

    let people = vec!["Sam", "mAtt", "kaTie", "joe"];
    for p in people {
        let user = find_user(p).map(|id| {User{user_id: id, name: p.to_string()}});
        match user {
            Some(user) => println!("{:?}", user),
            None => println!("User not found"),
        }
        // println!("{:?} with id {:?}", user.name, user.user_id);
    }
}
