use clap::Parser;
use cli_salad::create_fruit_salad;

#[derive(Parser)]
#[clap(
    version="1.0",
    author="Luis Ibarra <luissebastianibarrav@gmail.com>",
    about="Number of fruits to include in the salad"
)]
struct Opts {
    #[clap(short, long)]
    number: usize,
}

fn main() {
    let opts= Opts::parse();

    // Get the number of fruits the user requested
    let num_fruits = opts.number;

    // Create the fruit salad
    create_fruit_salad(num_fruits);

    // Print the fruit salad in human-readable format with a count of fruits used
    println!(
        "Created Fruit Salad with {} fruits: {:?}",
        num_fruits,
        create_fruit_salad(num_fruits)
    );
}
