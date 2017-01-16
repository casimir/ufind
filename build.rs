#[macro_use]
extern crate clap;

#[allow(dead_code)]
#[path = "src/app.rs"]
mod app;

use clap::Shell;

fn main() {
    let outdir = concat!(env!("CARGO_MANIFEST_DIR"), "/scripts/completion");
    std::fs::create_dir_all(&outdir).unwrap();

    let mut app = app::build();
    app.gen_completions(crate_name!(), Shell::Bash, &outdir);
    app.gen_completions(crate_name!(), Shell::Fish, &outdir);
    app.gen_completions(crate_name!(), Shell::Zsh, &outdir);
    app.gen_completions(crate_name!(), Shell::PowerShell, &outdir);
}
