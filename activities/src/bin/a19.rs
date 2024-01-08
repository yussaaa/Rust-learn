// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock

use std::collections::HashMap;

fn main() {
    //Defining the HashMap outside the main function is not allowed because Rust requires variables to have a 'static lifetime if they are accessed from multiple threads or if they are used in a static context. Since the main function can be executed concurrently by multiple threads, the HashMap needs to have a 'static lifetime to ensure thread safety. By defining the HashMap inside the main function, it is guaranteed to have a 'static lifetime within the scope of the function.
    let mut store = HashMap::new();
    store.insert("Chairs", 5);
    store.insert("Beds", 3);
    store.insert("Tables", 2);
    store.insert("Couches", 0);

    for (item, count) in &store {
        if *count == 0 {
            println!("{}: Out of stock", item);
        } else {
            println!("{}: {}", item, count);
        }
    }

    let total: i32 = store.values().sum();
    println!("Total items: {}", total);
}
