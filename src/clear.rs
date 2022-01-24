use clap::{App, AppSettings, Arg, ArgMatches};

use std::fs;
use std::path;

use glifparser::glif::mfek::traits::*;

pub fn clap_app() -> clap::App<'static> {
    App::new("CLEAR")
            .setting(AppSettings::DeriveDisplayOrder)
            .about("Delete all contours in glyph")
            .version("0.0.0")
            .author("Fredrick Brennan <copypasteⒶkittens.ph>; MFEK Authors")
            .arg(Arg::new("input")
                .long("input")
                .short('i')
                .takes_value(true)
                .required(true)
                .allow_invalid_utf8(true)
                .help("The path to the input UFO `.glif` file. (will be overwritten!)"))
            .arg(Arg::new("prune-contour-ops")
                .long("prune-contour-ops")
                .short('P')
                .takes_value(false)
                .required(false)
                .help("Prune contour ops?"))
}

pub fn cli(matches: &ArgMatches) {
    let path_string: path::PathBuf = matches.value_of_os("input").unwrap().into(); // required options shouldn't panic
    let prune_contour_ops = matches.is_present("prune-contour-ops");

    let ext = path_string.extension().map(|e|e.to_ascii_lowercase().to_string_lossy().to_string()).unwrap_or(String::from("glif"));
    match ext.as_str() {
        "glif" => {
            let mut glif: glifparser::Glif<()> =
                glifparser::read(&fs::read_to_string(&path_string).expect("Failed to read path file!"))
                    .expect("glifparser couldn't parse input path glif. Invalid glif?");
            glif.outline = None;
            glifparser::write_to_filename(&glif, path_string).unwrap();
        },
        "glifjson" => {
            let mut glif: glifparser::MFEKGlif<()> =
                serde_json::from_str(&fs::read_to_string(&path_string).expect("Could not open file"))
                    .expect("Could not deserialize JSON MFEKGlif");
            if prune_contour_ops {
                glif.downgrade_contour_ops();
            } else {
                glif.layers = vec![];
            }
            fs::write(&path_string, serde_json::to_vec_pretty(&glif).unwrap()).expect("Failed to write file");
        },
        _ => unreachable!()
    }
}
