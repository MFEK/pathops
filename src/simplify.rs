use std::fs;

use clap::{App, AppSettings, Arg, ArgMatches};
use glifparser::{read, Glif};
use MFEKmath::simplify::simplify;

pub fn cli(matches: &ArgMatches) {
    let path_string = matches.value_of("input").unwrap(); // required options shouldn't panic

    let out_string = matches.value_of("output").unwrap();
    let mut glif: Glif<()> =
        read(&fs::read_to_string(path_string).expect("Failed to read the path file!"))
            .expect("glifparser couldn't parse input path gliph. Invalid gliph?");
    let final_result = simplify(glif.outline.unwrap());
    glif.outline = Some(final_result);
    glifparser::write_to_filename(&glif, out_string).expect("Failed to write ");
}
pub fn clap_app() -> clap::App<'static> {
    App::new("SIMPLIFY")
        .setting(AppSettings::DeriveDisplayOrder)
        .setting(AppSettings::AllowNegativeNumbers)
        .about("Simplifies the given glif")
        .version("0.1.0")
        .author("T Prajwal Prabhu <prajwalprabhu.tellar@gmail.com>")
        .arg(
            Arg::new("input")
                .long("input")
                .short('i')
                .takes_value(true)
                .required(true)
                .help("The path to the input glif file."),
        )
        .arg(
            Arg::new("output")
                .long("output")
                .short('o')
                .required(true)
                .takes_value(true)
                .help("The path to the output glif file."),
        )
}
