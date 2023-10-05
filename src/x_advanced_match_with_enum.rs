enum Discount {
    Percent(i32),
    Flat(i32),
}

struct Ticket {
    event: String,
    price: f32,
}

fn main() {
    let flat = Discount::Flat(2);

    match flat {
        Discount::Flat(2) => println!("flat 2"),
        Discount::Flat(amount) => println!("flat discount of {:?}", amount),
        _ => (),
    };

    let concert = Ticket {
        event: "concert".to_string(),
        price: 50.0,
    };

    match concert {
        Ticket { price: 50.0, event } => {
            println!("event @ 50: {:?}", event);
        }
        Ticket { price, .. } => {
            println!("price: {:?}", price)
        }
    }
}