use chrono;
use clap::{Parser, Subcommand};
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};

#[derive(Parser)]
#[command(name = "flashcard")]
#[command(about = "A CLI flashcard application")]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long, default_value = "flashcards.json")]
    file: String,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new flashcard
    Add {
        /// The question for the flashcard
        question: String,
        /// The answer for the flashcard
        answer: String,
    },
    /// Start a quiz session
    Quiz,
    /// List all flashcards
    List,
    /// View a specific flashcard by ID
    View {
        /// The ID of the flashcard to view
        id: u32,
    },
    /// Delete a flashcard by ID
    Delete {
        /// The Id of the flashcard to delete
        id: u32,
    },
    /// Reset all card stadistics
    Reset,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Flashcard {
    pub id: u32,
    pub question: String,
    pub answer: String,
    pub metadata: CardMetadata,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CardMetadata {
    pub difficulty: Difficulty,
    pub times_reviewed: u32,
    pub correct_count: u32,
    pub last_reviewed: Option<String>, // We'll use simple string dates for now
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FlashcardDeck {
    pub cards: HashMap<u32, Flashcard>,
    pub next_id: u32,
}

impl FlashcardDeck {
    pub fn new() -> Self {
        FlashcardDeck {
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

    pub fn update_card_difficulty(&mut self, card_id: u32, difficulty: Difficulty, correct: bool) {
        if let Some(card) = self.cards.get_mut(&card_id) {
            card.metadata.difficulty = difficulty;
            card.metadata.times_reviewed += 1;
            if correct {
                card.metadata.correct_count += 1;
            }
            card.metadata.last_reviewed = Some(chrono::Utc::now().format("%Y-%m-%d").to_string());
        }
    }

    pub fn delete_card(&mut self, card_id: u32) -> bool {
        self.cards.remove(&card_id).is_some()
    }

    pub fn get_card(&self, card_id: u32) -> Option<&Flashcard> {
        self.cards.get(&card_id)
    }

    pub fn reset_all_stats(&mut self) {
        for card in self.cards.values_mut() {
            card.metadata = CardMetadata::default();
        }
    }

    pub fn get_random_cards_ids(&self) -> Vec<u32> {
        let mut cards_ids: Vec<u32> = self.cards.keys().copied().collect();
        let mut rng = rand::rng();
        cards_ids.shuffle(&mut rng);
        cards_ids
    }

    pub fn save_to_file(&self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json_data = serde_json::to_string_pretty(self)?;
        fs::write(filename, json_data)?;
        Ok(())
    }

    pub fn load_from_file(filename: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file_content = fs::read_to_string(filename)?;
        let deck: FlashcardDeck = serde_json::from_str(&file_content)?;
        Ok(deck)
    }
}

impl Default for CardMetadata {
    fn default() -> Self {
        CardMetadata {
            difficulty: Difficulty::Medium,
            times_reviewed: 0,
            correct_count: 0,
            last_reviewed: None,
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // Load existing deck or create new one
    let mut deck = if std::path::Path::new(&cli.file).exists() {
        FlashcardDeck::load_from_file(&cli.file)?
    } else {
        FlashcardDeck::new()
    };

    match &cli.command {
        Commands::Add { question, answer } => {
            let id = deck.add_card(question.clone(), answer.clone());
            deck.save_to_file(&cli.file)?;
            println!("Added flashcard #{}: {}", id, question);
        }
        Commands::List => {
            if deck.cards.is_empty() {
                println!("No flashcards found. Add some with 'flashcard add <question> <answer>'");
            } else {
                println!("Flashcards in deck ({}):", deck.cards.len());

                let mut cards: Vec<&Flashcard> = deck.cards.values().collect();
                cards.sort_by_key(|card| card.id);
                // for card in deck.cards.values() {
                //     println!("#{}: {} -> {}", card.id, card.question, card.answer);
                // }
                for card in cards {
                    let success_rate = if card.metadata.times_reviewed > 0 {
                        (card.metadata.correct_count as f64 / card.metadata.times_reviewed as f64)
                            * 100.0
                    } else {
                        0.0
                    };

                    let difficulty_emoji = match card.metadata.difficulty {
                        Difficulty::Easy => "🟢",
                        Difficulty::Medium => "🟡",
                        Difficulty::Hard => "🔴",
                    };

                    println!(
                        "#{} {} [{}] Success: {:.0}% ({}/{})",
                        card.id,
                        difficulty_emoji,
                        format!("{:.30}", card.question).trim(),
                        success_rate,
                        card.metadata.correct_count,
                        card.metadata.times_reviewed
                    );

                    if card.metadata.times_reviewed > 0 {
                        println!(
                            "    Last reviewed: {}",
                            card.metadata
                                .last_reviewed
                                .as_ref()
                                .unwrap_or(&"Never".to_string())
                        );
                    }
                    println!();
                }

                // Print deck stadistics
                let total_reviews: u32 =
                    deck.cards.values().map(|c| c.metadata.times_reviewed).sum();
                let total_correct: u32 =
                    deck.cards.values().map(|c| c.metadata.correct_count).sum();
                let overall_success = if total_reviews > 0 {
                    (total_correct as f64 / total_reviews as f64) * 100.0
                } else {
                    0.0
                };

                println!("📈 Deck Statistics:");
                println!("   Total cards: {}", deck.cards.len());
                println!("   Total reviews: {}", total_reviews);
                println!("   Overall success rate: {:.1}%", overall_success);
            }
        }
        Commands::Quiz => {
            if deck.cards.is_empty() {
                println!("No flashcards to quiz! Add some first.");
            } else {
                run_quiz(&mut deck)?;
                deck.save_to_file(&cli.file)?;
            }
        }
        Commands::View { id } => match deck.get_card(*id) {
            Some(card) => {
                println!("📄 Flashcard #{}:", card.id);
                println!("❓ Question: {}", card.question);
                println!("💡 Answer: {}", card.answer);
                println!();

                let difficulty_emoji = match card.metadata.difficulty {
                    Difficulty::Easy => "🟢 Easy",
                    Difficulty::Medium => "🟡 Medium",
                    Difficulty::Hard => "🔴 Hard",
                };
                println!("📊 Statistics:");
                println!("   Difficulty: {}", difficulty_emoji);
                println!("   Times reviewed: {}", card.metadata.times_reviewed);
                println!("   Correct answers: {}", card.metadata.correct_count);

                if card.metadata.times_reviewed > 0 {
                    let success_rate = (card.metadata.correct_count as f64
                        / card.metadata.times_reviewed as f64)
                        * 100.0;
                    println!("   Success rate: {:.1}%", success_rate);
                    println!(
                        "   Last reviewed: {}",
                        card.metadata
                            .last_reviewed
                            .as_ref()
                            .unwrap_or(&"Never".to_string())
                    );
                } else {
                    println!("   Success rate: Not yet reviewed");
                }
            }
            None => {
                println!("❌ Flashcard #{} not found.", id);
            }
        },
        Commands::Delete { id } => {
            if deck.delete_card(*id) {
                deck.save_to_file(&cli.file)?;
                println!("🗑️  Deleted flashcard #{}", id);
            } else {
                println!("❌ Flashcard #{} not found.", id);
            }
        }
        Commands::Reset => {
            if deck.cards.is_empty() {
                println!("❌ No flashcards to reset.");
            } else {
                print!("⚠️  Are you sure you want to reset all statistics? This cannot be undone. (y/N): ");
                io::stdout().flush().unwrap();

                let mut input = String::new();
                io::stdin().read_line(&mut input)?;

                if input.trim().to_lowercase() == "y" {
                    deck.reset_all_stats();
                    deck.save_to_file(&cli.file)?;
                    println!("🔄 Reset all flashcard statistics.");
                } else {
                    println!("❌ Reset cancelled.");
                }
            }
        }
    }

    Ok(())
}

fn run_quiz(deck: &mut FlashcardDeck) -> Result<(), Box<dyn std::error::Error>> {
    println!("🎯 Starting quiz! Press Enter to see the answer, then rate your performance:");
    println!("Ratings: (c)orrect + easy, (g)ot it but medium, (w)rong/hard, (q)uit\n");

    let cards = deck.get_random_cards_ids();
    let mut quiz_count = 0;
    let mut correct_count = 0;

    for card_id in cards {
        quiz_count += 1;
        let (question, answer) = {
            let card = &deck.cards[&card_id];
            (card.question.clone(), card.answer.clone())
        };

        println!("--- Card {}/{} ---", quiz_count, deck.cards.len());
        println!("❓ Question: {}", question);
        print!("Press Enter to reveal answer...");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        println!("✅ Answer: {}", answer);
        println!();

        loop {
            print!("Rate your performance (c/g/w/q): ");
            io::stdout().flush().unwrap();

            input.clear();
            io::stdin().read_line(&mut input)?;

            match input.trim().to_lowercase().as_str() {
                "c" => {
                    deck.update_card_difficulty(card_id, Difficulty::Easy, true);
                    correct_count += 1;
                    println!("✨ Marked as correct & easy!\n");
                    break;
                }
                "g" => {
                    deck.update_card_difficulty(card_id, Difficulty::Medium, true);
                    correct_count += 1;
                    println!("👍 Marked as correct but medium difficulty!\n");
                    break;
                }
                "w" => {
                    deck.update_card_difficulty(card_id, Difficulty::Hard, false);
                    println!("📚 Marked as hard - review this one more!\n");
                    break;
                }
                "q" => {
                    println!("Quiz ended early!");
                    print_quiz_summary(quiz_count - 1, correct_count);
                    return Ok(());
                }
                _ => {
                    println!("Invalid input! Use: c (correct/easy), g (got it/medium), w (wrong/hard), q (quit)");
                    continue;
                }
            }
        }
    }

    print_quiz_summary(quiz_count, correct_count);
    Ok(())
}

fn print_quiz_summary(total: usize, correct: usize) {
    println!("🎉 Quiz Complete!");
    println!(
        "📊 Results: {}/{} correct ({:.1}%)",
        correct,
        total,
        if total > 0 {
            (correct as f64 / total as f64) * 100.0
        } else {
            0.0
        }
    );
}
