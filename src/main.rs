#![feature(fn_traits, stmt_expr_attributes)]
#![allow(non_snake_case)] // for our name MFEKpathops

use clap::{App, AppSettings};
use env_logger;
#[allow(unused)]
mod validators;

mod boolean;
mod simplify;
fn main() {
    env_logger::init();
    #[allow(unused_mut)] // we actually use it if cfg(feature=fontforge)
    let mut argparser = App::new("MFEKpathops")
        .setting(AppSettings::SubcommandRequired)
        .setting(AppSettings::DisableVersionFlag)
        .setting(AppSettings::DisableHelpSubcommand)
        .author("Fredrick R. Brennan <copypasteⒶkittens⊙ph>; MFEK Authors")
        .about("A utility for applying path operations to contours (in UFO .glif format).")
        .subcommand(boolean::clap_app())
        .subcommand(simplify::clap_app());

    let matches = argparser.get_matches();

    match matches.subcommand_name() {
        Some("BOOLEAN") => boolean::cli(matches.subcommand_matches("BOOLEAN").unwrap()),
        Some("SIMPLIFY") => simplify::cli(matches.subcommand_matches("SIMPLIFY").unwrap()),
        _ => {
            unreachable!()
        }
    }
}
