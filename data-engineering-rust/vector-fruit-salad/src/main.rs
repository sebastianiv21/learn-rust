/*
This program creates a fruit salad by scrambling (shuffling) a list of fruit.
A vector is a growable array. It can grow or shrink in size and is one of the most
useful data structures in Rust. A vector is represented using the Vec<T> type.
*/

use rand::seq::SliceRandom;
use rand::thread_rng;
use std::io;

fn main() {
    let mut fruit = vec![
        "Orange".to_string(),
        "Fig".to_string(),
        "Pomegranate".to_string(),
        "Cherry".to_string(),
        "Apple".to_string(),
        "Pear".to_string(),
        "Peach".to_string(),
    ];

    let mut input = String::new();

    loop {
        input.clear();
        println!("Please type a fruit or 'shuffle' to continue");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let item = input.trim();

        if item.eq_ignore_ascii_case("shuffle") {
            break;
        }

        if !item.is_empty() {
            fruit.push(item.to_string());
            println!("You added {item} to your salad, yummy!");
        }
    }

    // shuffle the vector
    let mut rng = thread_rng();
    fruit.shuffle(&mut rng);

    // print out the fruit salad
    println!("Fruit Salad:");
    for (i, item) in fruit.iter().enumerate() {
        if i != fruit.len() - 1 {
            print!("{}, ", item);
        } else {
            println!("{}", item);
        }
    }

    let random_fruit = fruit.choose(&mut rng).unwrap();
    println!("Random fruit: {random_fruit}");

    add_random_fruits(&mut fruit, 4);

    println!("New salad {fruit:?}");
}

fn add_random_fruits(fruits: &mut Vec<String>, qty: i32) {
    let other_fruits = vec![
        "Strawberry".to_string(),
        "Pear".to_string(),
        "Grapes".to_string(),
        "Lemon".to_string(),
    ];

    let mut rng = thread_rng();

    let mut random_other_fruits = Vec::new();
    for _ in 0..qty {
        let rand_fruit = other_fruits.choose(&mut rng).unwrap().to_string();
        random_other_fruits.push(rand_fruit);
    }

    fruits.extend(random_other_fruits);
    // alternative
    // fruits.append(&mut random_other_fruits);
}
