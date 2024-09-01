fn main() {
    let message = "Name: Luis, Weight: ";
    let weight = 190.0;

    let kilos = weight / 2.2;
    println!("{}{}", message, kilos);

    let proceed = true;
    if proceed {
        println!("Proceeding!");
    } else {
        println!("Not proceeding!");
    }

    let height = 180;
    if height > 190 {
        println!("You're tall");
    } else if height > 160 {
        println!("Average height");
    } else {
        println!("You're short")
    }

    // assign variable in if conditional
    let mut grade = 5;
    grade = grade - 2;

    // kind of ternary
    let result = if grade == 5 {
        "GREAT!"
    } else if grade >= 3 {
        "Nice job!"
    } else {
        "Good try!"
    };

    println!("Result: {}", result);

    // better ternary
    let health = if height < 180 { "Good" } else { "Unknown" };

    println!("Heatlh: {}", health);

    // arrays cant change their length or data types, just the element's values
    // Declare array, initialize all values, compiler infers length = 7
    let days = [
        "Sunday",
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
    ];

    // Declare array, initialize all values to 0, length = 5
    let bytes = [0; 5];

    // a vector, unlike an array, can grow or shrink in size
    // Declare vector, initialize with three values
    let three_nums = vec![15, 3, 46];
    println!("Initial vector: {:?}", three_nums);

    // Declare vector, value = "0", length = 5
    let zeroes = vec![0; 5];
    println!("Zeroes: {:?}", zeroes);
    //
    // Create empty vector, declare vector mutable so it can grow and shrink
    let mut fruit = Vec::new();

    // Push values onto end of vector, type changes from generic `T` to String
    fruit.push("Apple");
    fruit.push("Banana");
    fruit.push("Cherry");
    println!("Fruits: {:?}", fruit);

    // Pop off value at end of vector
    // Call pop() method from inside println! macro
    println!("Pop off: {:?}", fruit.pop());
    println!("Fruits: {:?}", fruit);

    // Declare vector, initialize with three values
    let mut index_vec = vec![15, 3, 46];
    let three = index_vec[1];
    println!("Vector: {:?}, three = {}", index_vec, three);

    // Add 5 to the value at index 1, which is 5 + 3 = 8
    index_vec[1] = index_vec[1] + 5;
    println!("Vector: {:?}", index_vec);
}
