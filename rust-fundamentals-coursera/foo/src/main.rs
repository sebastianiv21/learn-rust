use std::io;

fn main() {
    println!("Hello, world!");

    let mut x = 0;

    loop {
        println!("x is {}", x);
        x += 1;
        if x == 10 {
            break;
        }
    }

    // let maybe_number: Option<Option<()>> = Some(None);
    let maybe_number = Some(42);
    if let Some(number) = maybe_number {
        println!("The number is {:?}", number);
    } else if let Some(42) = maybe_number {
        println!("The number is 42, oh my!");
    } else {
        println!("There is no number");
    }

    let mut y = 0;

    // while loop
    while y < 10 {
        println!("y is {}", y);
        y += 1;
    }

    let mut input = String::new();

    while input.trim() != "stop" {
        input.clear();
        println!("Please type something ('stop' to quit)");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        println!("You entered {}!", input.trim());
    }
    println!("Goodbye!");
}
