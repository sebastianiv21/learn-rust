use std::io;

fn main() {
    // // using the loop keyword is useful to avoid having to define the
    // // condition upfront or if the condition is met in the middle of the loop.
    // // It is also useful when you want to loop without knowing exactly when to stop
    // let mut x = 1;
    // // continue looping until x > 5
    // loop {
    //     println!("x is {}", x);
    //     x += 1;
    //     if x > 5 {
    //         break;
    //     }
    // }
    //
    // // there are other conditionals that we can explore in Rust. Like using 'if let'
    // // let maybe_number: Option<Option<()>> = Some(None);
    // let maybe_number = Some(Some(42));
    // if let Some(Some(number)) = maybe_number {
    //     println!("The number is {:?}", number);
    // } else if let Some(Some(42)) = maybe_number {
    //     println!("It's 42!");
    // } else {
    //     println!("There is no number!");
    // }
    //
    // // while loop
    // let mut i = 1;
    // while i < 5 {
    //     println!("Iterator: {}", i);
    //     i += 1;
    // }
    //
    // // this example is a useful application of 'while' because it allows to continue asking for
    // // user input until the user types a specific word (in this case , "stop").
    // let mut input = String::new();
    // while input.trim() != "stop" {
    //     input.clear();
    //     println!("Please enter a word (type 'stop' to exit):");
    //     io::stdin()
    //         .read_line(&mut input)
    //         .expect("Failed to read input");
    //     println!("You entered: {}", input);
    // }
    //
    // println!("Good bye!");

    // the for loop using a range. Note that you can use also `(1..10)` or `(1..=10)`
    for i in 1..10 {
        println!("i = {}", i);
    }

    for i in (1..=5).rev() {
        println!("i = {}", i);
    }

    // iterate in a vector
    let numbers = vec![1, 2, 3, 4, 5];
    for n in numbers {
        println!("{}", n);
    }

    // break and continue
    for i in 1..=10 {
        if i % 2 == 0 {
            // skip even numbers
            continue;
        }
        println!("i = {}", i);
        if i == 7 {
            // exit loop when i is 7
            break;
        }
    }

    // match control flow
    println!("Please enter a greeting: ");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Can't get the input");

    // use of match expression to pattern match against variable "name"
    match name.trim() {
        "Good Bye" => println!("Sorry to see you go."),
        "Hello" => println!("Hi, nice to meet you!"),
        // catch all, anything else
        _ => println!("I can't find a greeting, good bye."),
    }
}
