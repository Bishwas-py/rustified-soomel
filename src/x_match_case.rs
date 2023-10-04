fn mate_case_learn() {
    let username = "@compilxer";
    let mut is_admin = false;
    let mut user_state = "n.user";

    match username {
        "@admin" => {
            is_admin = true;
            user_state = "a.user";
        }
        "@compiler" => {
            is_admin = true;
            user_state = "c.user";
        }
        &_ => {
            is_admin = true;
            user_state = "b.user";
        }
    }

    println!("Is admin: {is_admin} and user state: {user_state}")
}
