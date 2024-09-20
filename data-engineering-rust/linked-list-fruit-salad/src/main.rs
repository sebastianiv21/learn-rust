/*
A LinkedList is a doubly-linked list, which means that each element in the list has a
pointer to the next element and the previous element.

A LinkedList has a higher memory overhead and worse cache locality than Vec or VecDeque, so it's typically
not the best choice unless you have a specific need for the properties of a linked list. In Rust, it's usually
better to use a Vec or VecDeque.

A great example of when to use a LinkedList is when you need to insert or remove elements from the middle of the list.
 */
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::LinkedList;

fn main() {
    let mut fruits: LinkedList<&str> = LinkedList::new();
    fruits.push_back("Arbutus");
    fruits.push_back("Loquat");
    fruits.push_back("Strawberry Tree Berry");

    // Please note that converting a LinkedList to a Vec and back to a LinkedList isn't a common operation in practice.
    // Shuffle the fruit
    let mut rng = thread_rng();
    let mut fruits: Vec<_> = fruits.into_iter().collect();
    fruits.shuffle(&mut rng);

    // Convert it back to a LinkedList
    let mut fruits: LinkedList<_> = fruits.into_iter().collect();
    fruits.push_front("Pomegranate");
    fruits.push_back("Fig");
    fruits.push_back("Orange");

    // Print out the fruit salad
    println!("Fruit Salad:");
    for (i, fruit) in fruits.iter().enumerate() {
        if i != fruits.len() - 1 {
            print!("{}, ", fruit);
        } else {
            print!("{}", fruit);
        }
    }
}
