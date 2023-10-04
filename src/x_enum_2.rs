enum Color {
    Red, Yellow, Blue
}

fn print_color(my_color: Color) {
    match my_color {
        Color::Red => {
            println!("RED COLOR.")
        }
        Color::Yellow => {
            println!("YELLOW COLOR.")
        }
        Color::Blue => {
            println!("BLUE COLOR")
        }
    }
}

fn enum_learn_2() {
    let color = Color::Blue;
    print_color(color)
}
