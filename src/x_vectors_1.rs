struct Test {
    scrore: i32
}

fn main() {
    let my_scores = vec![
        Test { scrore: 34 },
        Test { scrore: 38 },
        Test { scrore: 28 },
        Test { scrore: 87 },
    ];

    for test in my_scores {
        println!("Score = {:?}", test.scrore)
    }
}