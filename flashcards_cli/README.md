# Flashcards CLI

A command-line flashcard application built in Rust for creating, managing, and studying flashcards with spaced repetition features.

## Features

- **Add Flashcards**: Create new flashcards with questions and answers
- **Interactive Quiz**: Test your knowledge with randomized quiz sessions
- **Performance Tracking**: Track statistics including difficulty, success rate, and review history
- **Smart Difficulty System**: Cards are automatically categorized as Easy (🟢), Medium (🟡), or Hard (🔴) based on your performance
- **Persistent Storage**: All flashcards and statistics are saved to JSON files
- **Multiple Decks**: Support for custom deck files using the `-f` flag

## Installation

Make sure you have [Rust](https://www.rust-lang.org/tools/install) installed, then build the project:

```bash
cargo build --release
```

## Usage

### Basic Commands

Run the CLI using `cargo run --` followed by a command:

```bash
# Add a new flashcard
cargo run -- add "What is the capital of France?" "Paris"

# List all flashcards with statistics
cargo run -- list

# Start an interactive quiz session
cargo run -- quiz

# View details of a specific flashcard
cargo run -- view 1

# Delete a flashcard by ID
cargo run -- delete 1

# Reset all statistics (with confirmation)
cargo run -- reset
```

### Using Custom Deck Files

By default, flashcards are stored in `flashcards.json`. You can specify a different file:

```bash
# Use a custom deck file
cargo run -- -f my_cards.json add "What is 2+2?" "4"
cargo run -- -f my_cards.json quiz
```

### Quiz Session

During a quiz, you'll be presented with questions and can rate your performance:

- **c** - Correct & Easy (marks card as easy difficulty)
- **g** - Got it but Medium (marks card as medium difficulty) 
- **w** - Wrong/Hard (marks card as hard difficulty)
- **q** - Quit quiz early

The app tracks your performance and adjusts card difficulty accordingly.

## File Structure

- `src/main.rs` - Main application code
- `Cargo.toml` - Project dependencies and metadata
- `flashcards.json` - Default flashcard storage (created automatically)
- `my_cards.json` - Example custom deck file

## Dependencies

- **clap** - Command-line argument parsing
- **serde** - JSON serialization/deserialization
- **chrono** - Date handling for review tracking
- **rand** - Random card shuffling for quizzes

## Data Format

Flashcards are stored in JSON format with the following structure:

```json
{
  "cards": {
    "1": {
      "id": 1,
      "question": "What is 2 + 2?",
      "answer": "4",
      "metadata": {
        "difficulty": "Easy",
        "times_reviewed": 3,
        "correct_count": 2,
        "last_reviewed": "2025-07-26"
      }
    }
  },
  "next_id": 2
}
```

## Statistics Tracking

The app tracks comprehensive statistics for each card:

- **Difficulty Level**: Easy, Medium, or Hard based on performance
- **Times Reviewed**: Total number of times the card was shown in quizzes
- **Correct Count**: Number of times answered correctly
- **Success Rate**: Percentage of correct answers
- **Last Reviewed**: Date of last quiz session

