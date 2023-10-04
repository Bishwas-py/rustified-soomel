enum Direction {
    Up,
    Down,
    Left,
    Right
}


fn enum_learn() {
    let go = Direction::Left;

    match go {
        Direction::Up => {
            println!("go left")
        }
        Direction::Down => {
            println!("go down")
        }
        Direction::Left => {
            println!("go left")
        }
        Direction::Right => {
            println!("go right")
        }
    }

}
