fn main() {
    let my_numbers = vec![10, 20, 30, 40];
    for num in &my_numbers {
        if num == &30 {
            println!("thirty");
        } else {
            println!("{}", num)
        }
    }

    println!("len:  {}", my_numbers.len());
}