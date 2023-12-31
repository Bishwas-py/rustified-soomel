enum Access {
    Admin,
    Manager,
    User,
    Guest
}

fn main() {
    let access_level = Access::Admin;
    let can_access_file = match access_level {
        Access::Admin => true,
        _ => false
    };

    println!("Can access: {:?}", can_access_file)
}