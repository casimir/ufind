extern crate atty;
#[macro_use]
extern crate clap;

mod app;
mod digraph;

use std::io::{self, Write};

fn print_result(result: &str) {
    if atty::is(atty::Stream::Stdout) {
        println!("{}", result);
    } else {
        print!("{}", result);
        io::stdout().flush().unwrap();
    }
}

fn main() {
    let matches = app::build().get_matches();
    match matches.subcommand() {
        ("digraph", Some(digraph_matches)) => {
            if let Some(input) = digraph_matches.value_of("input") {
                if input.chars().count() == 2 {
                    match digraph::get_char(input) {
                        Some(c) => print_result(&format!("{}", c)),
                        None => std::process::exit(1),
                    }
                } else if input.chars().count() == 1 {
                    match digraph::get_digraph(&input.chars().next().unwrap()) {
                        Some(digraph) => print_result(&format!("{}", digraph)),
                        None => std::process::exit(1),
                    }
                } else {
                    std::process::exit(2)
                }
            }
        }
        ("", None) => (),
        _ => unreachable!(),
    }
}
