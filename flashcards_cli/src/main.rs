use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct Flashcard {
    id: u32,
    question: String,
    answer: String,
    metadata: CardMetadata,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CardMetadata {
    difficulty: Difficulty,
    times_reviewed: u32,
    correct_count: u32,
    last_reviewed: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FlashcardDeck {
    cards: HashMap<u32, Flashcard>,
    next_id: u32,
}

impl FlashcardDeck {
    pub fn new() -> Self {
        Self {
            cards: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn add_card(&mut self, question: String, answer: String) -> u32 {
        let card = Flashcard {
            id: self.next_id,
            question,
            answer,
            metadata: CardMetadata::default(),
        };

        let id = card.id;
        self.cards.insert(id, card);
        self.next_id += 1;
        id
    }

    pub fn save_to_file(&self, filename: &str) -> Result<(), Box<dyn Error>> {
        let json_data = serde_json::to_string_pretty(self)?;
        fs::write(filename, json_data)?;
        Ok(())
    }

    pub fn load_from_file(filename: &str) -> Result<Self, Box<dyn Error>> {
        let file_content = fs::read_to_string(filename)?;
        let deck: FlashcardDeck = serde_json::from_str(&file_content)?;
        Ok(deck)
    }
}

impl Default for CardMetadata {
    fn default() -> Self {
        Self {
            difficulty: Difficulty::Medium,
            times_reviewed: 0,
            correct_count: 0,
            last_reviewed: None,
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut deck = FlashcardDeck::new();

    // Add some cards
    deck.add_card("What is 2 + 2?".to_string(), "4".to_string());
    deck.add_card("Capital of France?".to_string(), "Paris".to_string());

    // Save to file
    deck.save_to_file("flashcards.json")?;
    println!("Saved deck with {} cards", deck.cards.len());

    // Load from file
    let loaded_deck = FlashcardDeck::load_from_file("flashcards.json")?;
    println!("Loaded deck with {} cards", loaded_deck.cards.len());

    Ok(())
}
