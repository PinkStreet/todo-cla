
enum args {
    test,
}

fn main() {
    let pattern = String::from(std::env::args().nth(1).expect("No pattern given"));

    match pattern {
        &"test"     => println!("test"),
        _           => println!("not test"),
    };
}
