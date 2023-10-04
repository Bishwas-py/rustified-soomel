struct GroceryItem {
    stock: i32,
    price: f64
}

fn main() {
    let cereal = GroceryItem{
        stock: 10,
        price: 2.1
    };

    println!("Stock: {:?}", cereal.stock);
    println!("Price: {:?}", cereal.price);
}