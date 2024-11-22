use std::process;
use std::env;
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Failed to parse arguments: {err}");
        process::exit(1);
    });


    if let Err(e) = minigrep::run(config) {
        eprintln!("Application Erorr: {e}");
        process::exit(1);
    }
}
