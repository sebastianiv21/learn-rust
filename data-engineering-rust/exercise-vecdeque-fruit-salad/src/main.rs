/*
This code starts with an initial VecDeque,
converts it to a Vec for shuffling, and then converts it back to a VecDeque.
After that, it pushes "Pomegranate" to the front of the deque, and "Fig" and "Cherry"
to the back of the deque. Finally, it prints out the final fruit salad.

A VecDeque is a double-ended queue, which means that you can push and pop from both ends
of the queue.
*/

use rand::seq::SliceRandom; // rand is a random number generation library in Rust
use rand::thread_rng;
use std::collections::VecDeque;

fn main() {
    let mut fruit: VecDeque<String> = VecDeque::new();
    fruit.push_back("Arbutus".to_string());
    fruit.push_back("Loquat".to_string());
    fruit.push_back("Strawberry Tree Berry".to_string());

    // Scramble (shuffle) the fruit
    let mut rng = thread_rng();
    let mut fruit: Vec<_> = fruit.into_iter().collect();
    fruit.shuffle(&mut rng);

    // Convert it back to VecDeque
    let mut fruit: VecDeque<_> = fruit.into_iter().collect();

    // add fruits from input
    let mut input = String::new();

    loop {
        input.clear();

        println!("Please type a fruit or 'end' to continue");
        std::io::stdin()
            .read_line(&mut input)
            .expect("Insert a valid string");

        let item = input.trim();

        if item.eq_ignore_ascii_case("end") {
            break;
        }

        if !item.is_empty() {
            fruit.push_back(item.to_string());
            println!("You added {item} to your salad");
        }
    }

    // Add fruits to the both ends of the queue after shuffling
    fruit.push_front("Pomegranate".to_string());
    fruit.push_back("Fig".to_string());
    fruit.push_back("Cherry".to_string());

    // Print out the fruit salad
    println!("Fruit Salad:");
    for (i, item) in fruit.iter().enumerate() {
        if i != fruit.len() - 1 {
            print!("{}, ", item);
        } else {
            println!("{}", item);
        }
    }

    let rand_fruit_vec: Vec<_> = fruit.clone().into_iter().collect();

    let rand_fruit = rand_fruit_vec.choose(&mut rng).unwrap();
    println!("Random fruit {rand_fruit}");
}
