use clap::{App, AppSettings, Arg, ArgMatches};
use glifparser::outline::RefigurePointTypes as _;
use MFEKmath::Fixup as _;

use std::fs;
use std::path;

pub fn clap_app() -> clap::App<'static> {
    App::new("REFIGURE")
            .setting(AppSettings::DeriveDisplayOrder)
            .about("Fix point type/handle colocation errors in .glif files.")
            .version("0.0.1")
            .author("Fredrick Brennan <copypasteⒶkittens.ph>; MFEK Authors")
            .arg(Arg::new("input")
                .long("input")
                .short('i')
                .takes_value(true)
                .required(true)
                .allow_invalid_utf8(true)
                .help("The path to the input UFO `.glif` file. (will be overwritten!)"))
            .arg(Arg::new("remove-single-points")
                .long("remove-single-points")
                .short('1')
                .takes_value(false)
                .required(false)
                .help("Remove contours with single points"))
}

pub fn cli(matches: &ArgMatches) {
    let path_string: path::PathBuf = matches.value_of_os("input").unwrap().into(); // required options shouldn't panic
    let retain_threshold = if matches.is_present("remove-single-points") {
        2
    } else {
        1
    };

    let ext = path_string.extension().map(|e|e.to_ascii_lowercase().to_string_lossy().to_string()).unwrap_or(String::from("glif"));
    match ext.as_str() {
        "glif" => {
            let mut glif: glifparser::Glif<()> =
                glifparser::read(&fs::read_to_string(&path_string).expect("Failed to read path file!"))
                    .expect("glifparser couldn't parse input path glif. Invalid glif?");
            glif.outline.as_mut().map(|o|o.assert_colocated_within(0.01));
            glif.outline.as_mut().map(|o|o.refigure_point_types());
            glif.outline.as_mut().map(|o|o.retain(|c|c.len() >= retain_threshold));
            glifparser::write_to_filename(&glif, path_string).unwrap();
        },
        "glifjson" => {
            let mut glif: glifparser::MFEKGlif<()> =
                serde_json::from_str(&fs::read_to_string(&path_string).expect("Could not open file"))
                    .expect("Could not deserialize JSON MFEKGlif");
            for layer in glif.layers.iter_mut() {
                layer.outline.retain(|c|c.inner.len() > 1);
                for outline in layer.outline.iter_mut() {
                    outline.inner.refigure_point_types();
                }
            }
            fs::write(&path_string, serde_json::to_vec_pretty(&glif).unwrap()).expect("Failed to write file");
        },
        _ => unreachable!()
    }
}
