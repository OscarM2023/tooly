use std::{env, process};
use tooly::Config;

fn main() {
    let configuration = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = tooly::run(configuration) {
        eprintln!("Execution error: {e}");
        process::exit(1);
    }
}
