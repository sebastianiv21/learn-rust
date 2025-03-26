/*
As with the VecDeque example, this code starts by creating a LinkedList of fruits,
converts it to a Vec for shuffling, and then converts it back to a LinkedList.
After the shuffling, it adds "Pomegranate", "Fig", and "Cherry" to the end of the list.
Finally, it prints out the final fruit salad.

This example shows how to use a LinkedList, but remember that LinkedList
has a higher memory overhead and worse cache locality than Vec or VecDeque,
so it's typically not the best choice unless you have a specific need for the properties
of a linked list. In Rust, it's usually better to use a Vec or VecDeque.

A LinkedList is a doubly-linked list, which means that each element in the list
has a pointer to the next element and the previous element.
A great example of when to use a LinkedList is when you need to insert or remove elements
from the middle of the list.
*/

use rand::seq::SliceRandom; // For shuffling and choosing a random fruit
use rand::thread_rng;
use std::collections::LinkedList;
use std::io;

fn main() {
    let mut fruit: LinkedList<&str> = LinkedList::new();
    fruit.push_back("Arbutus");
    fruit.push_back("Loquat");
    fruit.push_back("Strawberry Tree Berry");

    // Shuffle the fruit
    let mut rng = thread_rng();
    let mut fruit_vec: Vec<_> = fruit.into_iter().collect();
    fruit_vec.shuffle(&mut rng);

    // Convert back to LinkedList
    let mut fruit: LinkedList<_> = fruit_vec.into_iter().collect();

    // Add fruits at the front and back
    fruit.push_front("Pomegranate");
    fruit.push_back("Fig");
    fruit.push_back("Cherry");

    // Allow user to add a fruit at any position
    println!("Enter a fruit to add:");
    let mut new_fruit = String::new();
    io::stdin().read_line(&mut new_fruit).unwrap();
    let new_fruit = new_fruit.trim();

    println!("Enter a position (0 for front, {} for back):", fruit.len());
    let mut position = String::new();
    io::stdin().read_line(&mut position).unwrap();
    let position: usize = position.trim().parse().unwrap_or(fruit.len());

    insert_at(&mut fruit, new_fruit, position);

    // Select a random fruit
    let mut fruit: Vec<_> = fruit.into_iter().collect();
    if let Some(random_fruit) = fruit.iter().choose(&mut rng) {
        println!("Randomly selected fruit: {}", random_fruit);
    }
    let mut fruit: LinkedList<_> = fruit.into_iter().collect();

    // Remove a fruit from any position
    println!("Enter a position to remove a fruit:");
    let mut remove_position = String::new();
    io::stdin().read_line(&mut remove_position).unwrap();
    let remove_position: usize = remove_position.trim().parse().unwrap_or(0);

    if let Some(removed) = remove_at(&mut fruit, remove_position) {
        println!("Removed fruit: {}", removed);
    } else {
        println!("Invalid position!");
    }

    // Print final fruit salad
    println!("Final Fruit Salad: {:?}", fruit);
}

// Function to insert at any position
// add lifetimes
fn insert_at<'a>(list: &mut LinkedList<&'a str>, item: &'a str, position: usize) {
    let mut new_list = LinkedList::new();
    let mut index = 0;

    while let Some(f) = list.pop_front() {
        if index == position {
            new_list.push_back(item);
        }
        new_list.push_back(f);
        index += 1;
    }

    if position >= index {
        new_list.push_back(item);
    }

    *list = new_list;
}

// Function to remove from any position
// add lifetimes
fn remove_at<'a>(list: &mut LinkedList<&'a str>, position: usize) -> Option<&'a str> {
    let mut new_list = LinkedList::new();
    let mut index = 0;
    let mut removed = None;

    while let Some(f) = list.pop_front() {
        if index == position {
            removed = Some(f);
        } else {
            new_list.push_back(f);
        }
        index += 1;
    }

    *list = new_list;
    removed
}
