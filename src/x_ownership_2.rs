struct Grocery {
    quantity: i32,
    id: i32
}

fn display_quantity(grocery: &Grocery) {
    println!("Grocery Quantity:  {:?}", grocery.quantity)
}

fn display_id(grocery: &Grocery) {
    println!("Grocery ID:  {:?}", grocery.id)
}

fn main() {
    let grocery = Grocery {
        id: 2,
        quantity: 30
    };
    display_id(&grocery);
    display_quantity(&grocery);
}