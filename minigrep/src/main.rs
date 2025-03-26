use std::env;
use minigrep;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = minigrep::Config::new(&args);

    minigrep::run(config);
}

