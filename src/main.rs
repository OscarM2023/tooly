use std::{env, process};
use tooly::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let configuration = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = tooly::run(configuration) {
        println!("Execution error: {e}");
        process::exit(1);
    }
}