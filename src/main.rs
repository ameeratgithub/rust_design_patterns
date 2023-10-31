use std::process;

use design_patterns::Config;

fn main() {
    
    let config = Config::build().unwrap_or_else(|err: &str| {
        eprintln!("Unable to parse arguments: {err}");
        process::exit(1);
    });

    design_patterns::run(config).unwrap_or_else(|e|{
        eprintln!("Unable to run pattern: {e}");
        process::exit(1);
    });
    
}
