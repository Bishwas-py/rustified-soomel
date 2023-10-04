fn get_coords() -> (i32, i32) {
    (3, 2)
}

fn main() {
    let (x, y) = get_coords();
    if y > 5 {
        println!("{y} is greater than 5.")
    } else if y < 5 {
        println!("{y} is smaller than 5.")
    } else if y == 5 {
        println!("{y} is equal to 5.")
    }
}