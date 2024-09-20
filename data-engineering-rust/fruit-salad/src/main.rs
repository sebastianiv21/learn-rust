use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {
    let mut fruits = vec![
        "Orange",
        "Fig",
        "Pomegranate",
        "Cherry",
        "Apple",
        "Pear",
        "Peach",
    ];

    // Scramble (shuffle) the fruit
    let mut rng = thread_rng();
    fruits.shuffle(&mut rng);

    // Print out the fruit salad
    println!("Fruit salad:");
    for (i, fruit) in fruits.iter().enumerate() {
        if i != fruits.len() - 1 {
            print!("{}, ", fruit);
        } else {
            print!("{}", fruit);
        }
    }
}
