#![allow(non_snake_case)] // for our name MFEKpathops

use clap::{App, AppSettings};
use env_logger;
#[allow(unused)]
mod validators;

mod clear;
mod boolean;
mod fit_to_points;
mod refigure;
fn main() {
    env_logger::init();
    #[allow(unused_mut)] // we actually use it if cfg(feature=fontforge)
    let mut argparser = App::new("MFEKpathops")
        .setting(AppSettings::SubcommandRequired)
        .setting(AppSettings::DisableHelpSubcommand)
        .author("Fredrick R. Brennan <copypasteⒶkittens⊙ph>; MFEK Authors")
        .about("A utility for applying path operations to contours (in UFO .glif format).")
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand(boolean::clap_app())
        .subcommand(fit_to_points::clap_app())
        .subcommand(clear::clap_app())
        .subcommand(refigure::clap_app());

    let matches = argparser.get_matches();

    match matches.subcommand_name() {
        Some("BOOLEAN") => boolean::cli(matches.subcommand_matches("BOOLEAN").unwrap()),
        Some("FIT") => fit_to_points::cli(matches.subcommand_matches("FIT").unwrap()),
        Some("CLEAR") => clear::cli(matches.subcommand_matches("CLEAR").unwrap()),
        Some("REFIGURE") => refigure::cli(matches.subcommand_matches("REFIGURE").unwrap()),
        _ => {
            unreachable!()
        }
    }
}
