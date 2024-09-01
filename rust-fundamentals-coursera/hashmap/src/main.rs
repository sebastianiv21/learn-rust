use std::collections::HashMap;

fn main() {
    let mut reviews: HashMap<String, String> = HashMap::new();

    reviews.insert(
        String::from("Ancient Roman History"),
        String::from("Very accurate"),
    );
    reviews.insert(
        String::from("Cooking with Rhubard"),
        String::from("Sweet recipes"),
    );
    reviews.insert(
        String::from("Programming with Rust"),
        String::from("Great examples"),
    );

    let book: &str = "Programming with Rust";

    println!("Review for '{}': {:?}", book, reviews.get(book));

    let obsolete: &str = "Ancient Roman History";
    reviews.remove(obsolete);

    println!("Removed {}", obsolete);

    println!("Review for '{}': {:?}", obsolete, reviews.get(obsolete));
}
