use clap::Parser;
use patterns::{observer::ObserverPattern, strategy::StrategyPattern};
use puzzles::strategy_puzzle::StrategyPatternPuzzle;
use std::collections::HashMap;
use traits::DesignPattern;

pub mod patterns {
    pub mod observer;
    pub mod strategy;
}

pub mod traits;

pub mod puzzles {
    pub mod strategy_puzzle;
}

#[derive(Parser, Default, Debug)]
#[clap(
    author = "Ameer Hamza",
    version = "0.1.0",
    about = "Design Patterns in Rust"
)]
pub struct Config {
    #[clap(short = 'n', long = "name")]
    pub name: String,

    #[clap(short = 'p', long = "run_puzzle")]
    pub run_puzzle: bool,
}

impl Config {
    pub fn build() -> Result<Self, &'static str> {
        let args = Config::parse();

        Ok(args)
    }
}

struct App {
    patterns: HashMap<String, Box<dyn DesignPattern>>,
    puzzles: HashMap<String, Box<dyn DesignPattern>>,
}

impl App {
    fn new() -> Self {
        let mut patterns: HashMap<String, Box<dyn DesignPattern>> = HashMap::new();
        patterns.insert("observer".to_string(), Box::new(ObserverPattern));
        patterns.insert("strategy".to_string(), Box::new(StrategyPattern));

        let mut puzzles: HashMap<String, Box<dyn DesignPattern>> = HashMap::new();
        puzzles.insert("strategy".to_string(), Box::new(StrategyPatternPuzzle));

        App { patterns, puzzles }
    }
}

pub fn run(config: Config) -> Result<(), &'static str> {
    let app = App::new();

    let pattern_to_run: Option<&Box<dyn DesignPattern>>;

    if config.run_puzzle {
        pattern_to_run = app.puzzles.get(&config.name);
    } else {
        pattern_to_run = app.patterns.get(&config.name);
    }

    if let Some(pattern) = pattern_to_run {
        pattern.run();
        return Ok(());
    }

    Err("Pattern not found")
}
