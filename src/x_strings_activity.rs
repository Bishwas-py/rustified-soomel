struct Person {
    age: i32,
    name: String,
    color: String
}

fn main() {
    let people = vec![
        Person {
            age: 29,
            name: String::from("Bishwas"),
            color: String::from("blue")
        },
        Person {
            age: 27,
            name: String::from("Sneha"),
            color: String::from("pink")
        },
        Person {
            age: 2,
            name: String::from("Sam"),
            color: String::from("blue")
        },
    ];

    for person in people {
        if person.age <= 10 {
            print_name(&person.name);
            print_color(&person.color);
            println!("Age: {:?}", person.age);
        }
    }
}

fn print_name(name: &str) {
    println!("Name: {:?}", name);
}

fn print_color(color: &str) {
    println!("Color: {:?}", color);
}