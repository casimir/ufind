extern crate atty;
#[macro_use]
extern crate clap;

mod digraph;

use clap::{App, AppSettings, Arg, SubCommand};
use std::io::{self, Write};

fn build_cli() -> App<'static, 'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .subcommand(SubCommand::with_name("digraph")
            .setting(AppSettings::AllowLeadingHyphen)
            .about("Digraph lookup and resolution")
            .arg(Arg::with_name("input")
                .takes_value(true)
                .required(true)
                .help("The search term, either a digraph sequence or a character")))
}

fn print_result(result: &str) {
    if atty::is(atty::Stream::Stdout) {
        println!("{}", result);
    } else {
        print!("{}", result);
        io::stdout().flush().unwrap();
    }
}

fn main() {
    let matches = build_cli().get_matches();
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
