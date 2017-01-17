use clap::{App, AppSettings, Arg, ArgGroup, SubCommand};

pub fn build() -> App<'static, 'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .subcommand(SubCommand::with_name("digraph")
            .setting(AppSettings::AllowLeadingHyphen)
            .about("Digraph lookup and resolution")
            .arg(Arg::with_name("convert")
                .short("c")
                .takes_value(true)
                .help("Converts a digraph sequence or a character to the other"))
            .arg(Arg::with_name("filter")
                .short("f")
                .takes_value(true)
                .help("Prints information about matching digraphs"))
            .group(ArgGroup::with_name("modes").args(&["convert", "filter"]).required(true)))
}
