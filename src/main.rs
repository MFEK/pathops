#![allow(non_snake_case)] // for our name MFEKpathops

use clap::{App, AppSettings};
use env_logger;
#[allow(unused)]
mod validators;
mod bin_subcommands;

//pub mod simplify;

fn main() {
    env_logger::init();
    #[allow(unused_mut)] // we actually use it if cfg(feature=fontforge)
    let mut argparser = App::new("MFEKpathops")
        .setting(AppSettings::SubcommandRequired)
        .setting(AppSettings::DisableHelpSubcommand)
        .author("Fredrick R. Brennan <copypasteⒶkittens⊙ph>; MFEK Authors")
        .about("A utility for applying path operations to contours (in UFO .glif format).")
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand(bin_subcommands::boolean())
        .subcommand(bin_subcommands::fit_to_points())
        .subcommand(bin_subcommands::clear())
        //.subcommand(simplify::clap_app());
        .subcommand(bin_subcommands::refigure())
        .subcommand(bin_subcommands::nudge());

    let matches = argparser.get_matches();

    match matches.subcommand_name() {
        Some("BOOLEAN") => bin_subcommands::boolean_cli(matches.subcommand_matches("BOOLEAN").unwrap()),
        Some("FIT") => bin_subcommands::fit_to_points_cli(matches.subcommand_matches("FIT").unwrap()),
        Some("CLEAR") => bin_subcommands::clear_cli(matches.subcommand_matches("CLEAR").unwrap()),
        Some("REFIGURE") => bin_subcommands::refigure_cli(matches.subcommand_matches("REFIGURE").unwrap()),
        Some("NUDGE") => bin_subcommands::nudge_cli(matches.subcommand_matches("NUDGE").unwrap()),
        //Some("SIMPLIFY") => simplify::cli(matches.subcommand_matches("SIMPLIFY").unwrap()),
        _ => {
            unreachable!()
        }
    }
}
