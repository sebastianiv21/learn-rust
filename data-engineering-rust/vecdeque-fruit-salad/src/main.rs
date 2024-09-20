use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::VecDeque;

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
    for (i, fruit) in fruits.iter().enumerate() {
        if i != fruits.len() - 1 {
            print!("{}, ", fruit);
        } else {
            print!("{}", fruit);
        }
    }
}
