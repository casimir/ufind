use clap::{App, AppSettings, Arg, SubCommand};

pub fn build() -> App<'static, 'static> {
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
