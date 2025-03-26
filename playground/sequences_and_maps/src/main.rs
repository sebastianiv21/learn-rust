use std::collections::{HashMap, VecDeque};

fn main() {
    // Vector: Growable array, similar to Python list
    // create a vector
    let mut fruits = vec!["apple", "pineapple", "kiwi"];

    // add element
    fruits.push("orange");

    // print
    println!("Fruits vector: {:?}", fruits);

    // VecDeque: Double-ended queue with fast appends/pops on both ends
    // create vecdeque
    let mut fruit_deque = VecDeque::new();

    // push back
    fruit_deque.push_back("apple");

    // push front
    fruit_deque.push_front("cherry");

    // print
    println!("Fruit deque: {:?}", fruit_deque);

    // Linked List: List with efficient inserts/removals in middle but slower indexing

    // HashMap: Key-Value store, similar to Python dictionary
    // create map
    let mut fruit_calories = HashMap::new();

    // insert
    fruit_calories.insert("apple", 95);

    // print
    println!("Apple calories: {}", fruit_calories["apple"]);

    // Sequence: Immutable ordered collection indexed by position, similar to Pyton tuple
}
