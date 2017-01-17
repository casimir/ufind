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
            if digraph_matches.is_present("filter") {
                for it in digraph::filter(digraph_matches.value_of("filter").unwrap()) {
                    println!("{}{}\t{}", it.sequence[0], it.sequence[1], it.character)
                }
            } else if let Some(input) = digraph_matches.value_of("convert") {
                match digraph::convert(input) {
                    Some(out) => print_result(&out[..]),
                    None => std::process::exit(1),
                }
            }
        }
        ("", None) => (),
        _ => unreachable!(),
    }
}
