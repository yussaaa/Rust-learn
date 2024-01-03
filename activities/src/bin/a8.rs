// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

struct Drink{
    flavor: Flavors,
    fluid_ounces: f64,  
}
enum Flavors{
    Orange,
    Grape,
    Apple,
    Coke
}

fn print_drink(drink: Drink){
    match drink.flavor{
        Flavors::Orange => println!("Orange"),
        Flavors::Grape => println!("Grape"),
        Flavors::Apple => println!("Apple"),
        Flavors::Coke => println!("Coke"),
    }
    
    println!("{} ounces", drink.fluid_ounces);
}

fn main() {
    let apple_drink = Drink{flavor: Flavors::Apple, fluid_ounces: 12.0};
    print_drink(apple_drink);
}
