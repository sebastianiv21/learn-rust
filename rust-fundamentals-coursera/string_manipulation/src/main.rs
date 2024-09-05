fn longest_word(sentence: &str) -> &str {
    let mut longest = "";
    for word in sentence.split_whitespace() {
        if word.len() > longest.len() {
            longest = word;
        } else {
            continue;
        }
    }

    longest
}

fn main() {
    let sentence = "the quick brown fox jumps over the lazy dog".to_string();
    // Use slicing to get the first three characters of the sentence
    //println!("{}", &sentence[0..=4]);

    // concatenate using format!
    let description = format!("Title: Quick story\n{}", sentence);
    //println!("{}", description);

    // iterate over the characters in the sentence
    // for c in sentence.chars() {
    //     match c {
    //         'a' | 'e' | 'i' | 'o' | 'u' => println!("Got a vowel!"),
    //         _ => continue,
    //     }
    // }

    // Split and collect into a vector
    // let words: Vec<&str> = sentence.split_whitespace().collect();
    // let words = sentence.split(' ').collect::<Vec<&str>>();
    //println!("{:?}", words);

    // let reversed = sentence.chars().rev().collect::<String>();
    // println!("{}", reversed);

    let mut a = 0;
    let mut e = 0;
    let mut i = 0;
    let mut o = 0;
    let mut u = 0;

    for c in sentence.chars() {
        match c {
            'a' => a += 1,
            'e' => e += 1,
            'i' => i += 1,
            'o' => o += 1,
            'u' => u += 1,
            _ => continue,
        }
    }

    println!("a: {}, e: {}, i: {}, o: {}, u: {}", a, e, i, o, u);

    println!("The longest word is: {}", longest_word(&sentence));
}
