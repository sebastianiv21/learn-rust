use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::VecDeque;
use std::io;

fn main() {
    let mut fruits = VecDeque::new();
    fruits.push_back("Arbutus");
    fruits.push_back("Loquat");
    fruits.push_back("Strawberry Tree Berry");

    // Scramble (shuffle) the fruit
    let mut rng = thread_rng();
    let mut fruits: Vec<_> = fruits.into_iter().collect();
    fruits.shuffle(&mut rng);

    // Convert it back to VecDeque
    let mut fruits: VecDeque<_> = fruits.into_iter().collect();

    // Add fruits to the both ends of the queue after shuffling
    fruits.push_front("Pomegranate");
    fruits.push_back("Fig");
    fruits.push_back("Cherry");

    // Print out the fruit salad
    println!("Fruit Salad:");
    // for (i, fruit) in fruits.iter().enumerate() {
    //     if i != fruits.len() - 1 {
    //         print!("{}, ", fruit);
    //     } else {
    //         println!("{}", fruit);
    //     }
    // }
    print_fruit_salad(&fruits);

    // Choose a random fruit
    let fruits: Vec<_> = fruits.into_iter().collect();
    if let Some(&random_fruit) = fruits.choose(&mut rng) {
        println!("Random fruit: {}", random_fruit);
    }

    let mut fruits: VecDeque<_> = fruits.into_iter().collect();
    let mut input = String::new();
    println!("Type a fruit to add to the front:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let new_fruit = input.trim();
    fruits.push_front(new_fruit);

    input.clear();
    println!("Type a fruit to add to the back:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let new_fruit = input.trim();
    fruits.push_back(new_fruit);

    print_fruit_salad(&fruits);

    println!("Type b or f to remove a fruit from the back or front:");
    input.clear();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let choice = input.trim();

    match choice {
        "b" => {
            if let Some(removed) = fruits.pop_back() {
                println!("Removed: {}", removed);
            }
        }
        "f" => {
            if let Some(removed) = fruits.pop_front() {
                println!("Removed: {}", removed);
            }
        }
        _ => println!("Invalid choice"),
    }
}

fn print_fruit_salad(fruit: &VecDeque<&str>) {
    if fruit.is_empty() {
        println!("(vacío)");
    } else {
        println!("{}", fruit.iter().cloned().collect::<Vec<_>>().join(", "));
    }
}
