#![allow(dead_code, unused_imports)]
mod adapters;
mod cli;
mod contracts;
mod output;
mod repositories;
mod resolve;
mod validate;

fn main() {
    if let Err(err) = cli::run() {
        eprintln!("error: {err}");
        std::process::exit(1);
    }
}
