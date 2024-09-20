use std::collections::HashMap;
use chrono::{Datelike, Local};

fn init_languages() -> HashMap<String, i32>{
    let mut languages = HashMap::new();

    languages.insert("JavaSript".to_string(), 1995);
    languages.insert("HTML/CSS".to_string(), 1990);
    languages.insert("Python".to_string(), 1991);
    languages.insert("C++".to_string(), 1985);
    languages.insert("Java".to_string(), 1995);

    languages
}

fn calculate_weights(years_active: &mut HashMap<String, i32>) -> HashMap<String, i32>{
    // Subtract the creation year from the current year to get the number of years active
    let current_year = Local::now().year();
    for year in years_active.values_mut() {
        *year = current_year - *year;
    }

    let min_year = *years_active.values().min().unwrap_or(&0);
    let max_year = *years_active.values().max().unwrap_or(&0);

    let mut weights = HashMap::new();

    for (language, &year) in years_active.iter() {
        let normalized_year = (year - min_year) as f64 / (max_year - min_year) as f64;
        let weight = (normalized_year * 99.0) as i32 + 1; // weight between 1 and 100
        weights.insert(language.to_string(), weight);
    }

    weights
}

fn main() {
    let mut languages = init_languages();
    let weights = calculate_weights(&mut languages);

    println!("Language weighing from 1-100 by age (1 is newest and 100 is oldest):");
    for (language, weight) in &weights {
        println!("{language}: {weight}");
    }
}
