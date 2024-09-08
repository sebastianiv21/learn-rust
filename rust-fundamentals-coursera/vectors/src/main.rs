fn get_item(index: usize) {
    //let index = 3; // this looks like an unsigned integer, but it's actually a usize
    let vec = vec![1, 2, 3, 4, 5];

    // Retrieve a value at a specific index
    let value = vec.get(index).unwrap();

    // print the value
    println!("The value at index {} is {:?}", index, value);
}

fn sum_vector(numbers: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for n in numbers {
        sum += n;
    }
    sum
}

fn insert_first_last<T: Copy>(v: &mut Vec<T>, element: T) -> &mut Vec<T> {
    v.insert(0, element);
    v.push(element);
    v
}

fn main() {
    let vec = vec![1, 2, 3, 4, 5];
    get_item(3);

    // Retrieve a value at a specific index
    let third_value = vec[2];
    println!("The third value in the vector is: {}", third_value);

    // Retrieve the last value
    let last_value = vec.last().unwrap();
    //println!("The last value in the vector is: {}", last_value);

    // Retrieve the first value using pattern matching
    match vec.first() {
        Some(first_value) => println!("The first value in the vector is: {}", first_value),
        None => println!("The vector is empty!"),
    }

    let sum = sum_vector(&vec);

    println!("The sum of the vector is {}", sum);

    // Manage elements in a vector
    let mut v = vec![1, 2, 3];
    v.push(4);
    //println!("{:?}", v); // Output: [1, 2, 3, 4]

    // extend adds each element of the given slice to the vector
    let more_numbers = vec![5, 6];
    v.extend(more_numbers);
    //println!("{:?}", v);

    // append adds the given vector to the vector, requires the vector to be mutable
    let mut other_numbers = vec![7, 8];
    v.append(&mut other_numbers);
    // println!("{:?}", v);

    // insert items at a given index
    v.insert(0, 0);
    // println!("{:?}", v); // Output: [0, 1, 2, 3, 4, 5, 6, 7, 8]

    let mut my_vec = vec![0, 1, 2];
    let new_vec = insert_first_last(&mut my_vec, 4);
    println!("After inserting in first and last position: {:?}", new_vec);

    let fruits = vec!["banana", "apple", "coconut", "orange", "strawberry"];
    for &index in [0, 2, 99].iter() {
        match fruits.get(index) {
            Some(&"coconut") => println!("Coconuts are awesome!!!"),
            Some(fruit_name) => println!("It's a delicious {}!", fruit_name),
            None => println!("There is no fruit! :("),
        }
    }
}
