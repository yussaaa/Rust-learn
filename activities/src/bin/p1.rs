// Project 1: Interactive bill manager
//
// Summary:
//   Create a command line bills/expenses manager that runs
//   interactively. This mini project brings together many of
//   the concepts learn thus far into a single application.
//
//   The user stories/requirements are split into stages.
//   Fully implement each stage as a complete working program
//   before making changes for the next stage. Leverage the
//   compiler by using `cargo check --bin p1` when changing
//   between stages to help identify adjustments that need
//   to be made.
//
// User stories:
// * Stage 1:
//   - I want to add bills, including the name and amount owed.
//   - I want to view existing bills.
// * Stage 2:
//   - I want to remove bills.
// * Stage 3:
//   - I want to edit existing bills.
//   - I want to go back if I change my mind.
//
// Tips:
// * Use the loop keyword to create an interactive menu.
// * Each menu choice should be it's own function, so you can work on the
//   the functionality for that menu in isolation.
// * A vector is the easiest way to store the bills at stage 1, but a
//   hashmap will be easier to work with at stages 2 and 3.

use std::collections::HashMap;
use std::io;

fn get_input() -> Option<String> {
    let mut input = String::new();

    while io::stdin().read_line(&mut input).is_err() {
        println!("Error reading input. Please try again.");
        input.clear();
    }
    let input = input.trim().to_lowercase().to_string();
    if input == "" {
        return None;
    }
    return Some(input);
}

#[derive(Debug, Clone)]
pub struct Bill {
    name: String,
    amount: f64,
}

pub struct Bills {
    inner: HashMap<String, Bill>,
}

impl Bills {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    pub fn add(&mut self, bill: Bill) {
        self.inner.insert(bill.name.to_string(), bill);
    }

    pub fn remove(&mut self, name: String) -> bool {
        self.inner.remove(&name).is_some()
    }

    pub fn edit(&mut self, name: String, amount: f64) -> bool {
        // if let Some(x) = self.inner.get_mut(&name) {
        //     x.amount = amount;
        // }
        match self.inner.get_mut(&name) {
            Some(x) => {
                x.amount = amount;
                true
            }
            None => false,
        }
    }

    // pub fn get(&self, index: usize) -> Option<&Bill> {
    //     self.inner.get(index)
    // }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn get_all(&self) -> Vec<&Bill> {
        self.inner.values().collect()
    }
}

mod menu {
    use crate::{get_input, Bill, Bills};

    fn get_bill_amount() -> Option<f64> {
        println!("Bill amount: ");
        loop {
            let amount = match get_input() {
                Some(amount) => amount,
                None => {
                    println!("No amount entered!");
                    return None;
                }
            };

            if &amount == "" {
                return None;
            }

            // let parse_amount: Result<f64, _> = amount.parse();  //  ZTM solution
            // match parse_amount {

            //  match amount.parse() {  // GPT short answer

            let parse_amount = match amount.parse() {
                // My answer after experimenting
                Ok(amount) => return Some(amount),
                Err(_) => println!("Invalid amount entered! Try again!!"),
                // {
                //     println!("Invalid amount entered! Try again!!");
                //     return None;
                // }
            };
        }
    }

    pub fn add_bill(bills: &mut Bills) {
        println!("Bill name: ");
        let name = match get_input() {
            Some(name) => name,
            None => {
                println!("No name entered!");
                return;
            }
        };

        let parse_amount = match get_bill_amount() {
            Some(amount) => amount,
            None => return,
        };
        let bill = Bill {
            name: name,
            amount: parse_amount,
        };
        bills.add(bill);
    }

    pub fn view_bills(bills: &Bills) {
        println!("");
        println!("== Bills ==");
        println!("");
        for bill in bills.get_all() {
            // println!("{}: {} - ${}", index + 1, bill.name, bill.amount);
            println!("{:?}", bill);
        }
        println!("");
    }

    pub fn remove_bill(bills: &mut Bills) {
        view_bills(&bills);
        println!("Enter bill name to remove: ");
        let name = match get_input() {
            Some(name) => name,
            None => {
                println!("No name entered!");
                return;
            }
        };
        if bills.remove(name) {
            println!("Bill removed!");
        } else {
            println!("Bill not found!");
        }
        view_bills(&bills);
    }

    pub fn edit_bill(bills: &mut Bills) {
        view_bills(&bills);
        println!("Enter bill name to edit: ");
        let name = match get_input() {
            Some(name) => name,
            None => {
                println!("No name entered!");
                return;
            }
        };
        let parse_amount = match get_bill_amount() {
            Some(amount) => amount,
            None => return,
        };
        if bills.edit(name, parse_amount) {
            println!("Bill edited!");
        } else {
            println!("Bill not found!");
        };
        view_bills(&bills);
    }
}

enum BillManager {
    Add,
    View,
    Remove,
    Edit,
    Quit,
}

impl BillManager {
    fn from_str(input: &str) -> Option<BillManager> {
        match input {
            "1" => Some(BillManager::Add),
            "2" => Some(BillManager::View),
            "3" => Some(BillManager::Remove),
            "4" => Some(BillManager::Edit),
            "6" => Some(BillManager::Quit),
            _ => None,
        }
    }

    fn show() {
        println!("");
        println!("== Bill Manager ==");
        println!("");
        println!("1) Add bill");
        println!("2) View bills");
        println!("3) Remove bill");
        println!("4) Edit bill");
        println!("6) Quit");
        println!("");
    }
}

fn run_program() -> Option<()> {
    use menu::*;
    let mut bills = Bills::new();

    loop {
        BillManager::show();
        println!("Select from menu: ");
        let input = get_input()?;

        match BillManager::from_str(&input) {
            Some(BillManager::Add) => add_bill(&mut bills),
            Some(BillManager::View) => view_bills(&bills),
            Some(BillManager::Remove) => remove_bill(&mut bills),
            Some(BillManager::Edit) => edit_bill(&mut bills),
            Some(BillManager::Quit) => break,
            None => println!("Invalid input! Try again!!"),
        }
    }
    None
}

fn main() {
    // let i = get_input();
    // println!("{:?}", i);

    run_program();
}
