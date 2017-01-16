#[macro_use]
extern crate clap;

use clap::{App, AppSettings, Arg, SubCommand};

mod digraph;

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

fn main() {
    let matches = build_cli().get_matches();
    match matches.subcommand() {
        ("digraph", Some(digraph_matches)) => {
            if let Some(input) = digraph_matches.value_of("input") {
                if input.chars().count() == 2 {
                    match digraph::get_char(input) {
                        Some(c) => print!("{}", c),
                        None => std::process::exit(1),
                    }
                } else if input.chars().count() == 1 {
                    match digraph::get_digraph(&input.chars().next().unwrap()) {
                        Some(digraph) => print!("{}", digraph),
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
