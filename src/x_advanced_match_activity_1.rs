
enum Ticket {
    Backstage(String, i32),
    Vip(String, i32),
    Standard(i32)
}

fn main() {
    let tickets = vec![
        Ticket::Backstage("Binay".to_owned(), 400),
        Ticket::Vip("Bishwas".to_owned(), 200),
        Ticket::Standard(50)
    ];

    for ticket in tickets {
        match ticket {
            Ticket::Backstage(name, price) => {
                println!("A backstage ticket bought for {:?} by {:?}", price, name);
            }
            Ticket::Vip(name, price) => {
                println!("A Vip ticket bought for {:?} by {:?}", price, name);
            }
            Ticket::Standard(price) => {
                println!("Just a Standard ticket for {:?} by", price);
            }
        }
    }
}

