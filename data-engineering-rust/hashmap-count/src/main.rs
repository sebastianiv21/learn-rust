// This example code counts the frequency of each number in the vector

use std::collections::HashMap;

fn count_frequency(numbers: Vec<i32>) -> Vec<(i32, i32)> {
    let mut frequencies = HashMap::new();

    // counts every number in the vector
    for num in numbers {
        // same operation
        // frequencies.entry(num).and_modify(|counter| *counter += 1).or_insert(1);
        let frequency = frequencies.entry(num).or_insert(0);
        *frequency += 1;
    }

    let mut result = Vec::new();

    for (num, frequency) in frequencies {
        result.push((num, frequency));
    }

    result
}
fn main() {
    let numbers = vec![1,2,3,2,1,2,3,4,5,4,4,3,2];
    let result = count_frequency(numbers);

    println!("The result is {result:?}");
}
