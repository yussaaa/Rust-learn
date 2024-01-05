// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info
struct Ticket {
    ticket_type: TicketType,
    price: f32,
}

enum TicketType {
    Backstage(String),
    Vip(String),
    Standard,
}

fn printTicketInfo(ticket: &Ticket) {
    match &ticket.ticket_type {
        TicketType::Backstage(name) => {
            println!("Backstage ticket for {} is ${}", name, ticket.price)
        }
        TicketType::Vip(name) => println!("Vip ticket for {} is ${}", name, ticket.price),
        TicketType::Standard => println!("Standard ticket is ${}", ticket.price),
    }
}

fn main() {
    // let t = Ticket {
    //     ticket_type: TicketType::Backstage("John Doe".to_string()),
    //     price: 100.0,
    // };
    // let t2 = Ticket {
    //     ticket_type: TicketType::Standard,
    //     price: 50.0,
    // };
    // let tickets = vec![t, t2];
    let tickets = vec![
        Ticket {
            ticket_type: TicketType::Backstage("John Doe".to_string()),
            price: 100.0,
        },
        Ticket {
            ticket_type: TicketType::Standard,
            price: 50.0,
        },
        Ticket {
            ticket_type: TicketType::Vip("Sally".to_string()),
            price: 150.0,
        },
    ];

    for ticket in tickets {
        printTicketInfo(&ticket);
    }
    // printTicketInfo(&t);
    // printTicketInfo(&t2);
}

// Note: It's easier to only use enum to save the tiack info as it can save more than one type of data
// enum Ticket {
//     Backstage(String, f32),
//     Vip(String, f32),
//     Standard(f32),
// }
