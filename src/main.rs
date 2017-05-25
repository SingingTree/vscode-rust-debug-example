fn another_function() {
    let message = "I'm a message in another function";
    println!("{}", message);
}

fn yet_another_function() -> u64 {
    let number = 2;
    let another_number = 5;
    number + another_number
}

fn main() {
    println!("Hello, world!");
    another_function();
    let number = yet_another_function();
    println!("{}", number);
}
