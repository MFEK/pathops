use std::fs;

use super::*;

use glifparser::outline::RefigurePointTypes as _;
use mfekpathops::boolean::{EngineOp, FloPathOp, apply_flo, apply_skia, skia};

pub fn clap_app() -> clap::App<'static> {
    App::new("BOOLEAN")
            .setting(AppSettings::DeriveDisplayOrder)
            .setting(AppSettings::AllowNegativeNumbers)
            .about("Applies a boolean (union/intersect/difference/XOR…) operation to a glyph in UFO .glif format. Some of the algorithms use Skia, others use flo_curves.")
            .version("0.1.0")
            .author("Fredrick Brennan <copypasteⒶkittens.ph>; Skia Authors; Andrew Hunter (flo_curves.rs); MFEK Authors")
            .arg(Arg::new("pathop")
                .long("pathop")
                .short('p')
                .takes_value(true)
                .possible_values(&["difference", "intersect", "union", "xor", "reverse_difference", "add", "flo_intersect", "remove_interior", "remove_overlapping", "sub"])
                .hide_possible_values(true)
                .default_value("union")
                .help("Boolean operation to apply. [skia values: difference, intersect, union, xor, reverse_difference] [flo_curves values: add, flo_intersect, remove_interior, remove_overlapping, sub]"))
            .arg(Arg::new("input")
                .long("input")
                .short('i')
                .takes_value(true)
                .required(true)
                .help("The path to the input glif file."))
            .arg(Arg::new("operand")
                .long("operand")
                .short('O')
                .takes_value(true)
                .forbid_empty_values(true)
                .help("The path to the glif file that will act as the operand to the boolean operation. (skia: required if <pathop> not union.)  (flo_curves: only used if mode is flo_intersect, remove_interior or remove_overlapping)"))
            .arg(Arg::new("output")
                .long("output")
                .short('o')
                .required(true)
                .takes_value(true)
                .help("The path to the output glif file."))
}

pub fn cli(matches: &ArgMatches) {
    let path_string = matches.value_of("input").unwrap(); // required options shouldn't panic
    let operand_string = matches.value_of("operand");
    let out_string = matches.value_of("output").unwrap();

    let engine_op = match matches.value_of("pathop").unwrap() {
        "difference" => EngineOp::Skia(skia::PathOp::Difference),
        "intersect" => EngineOp::Skia(skia::PathOp::Intersect),
        "union" => EngineOp::Skia(skia::PathOp::Union),
        "xor" => EngineOp::Skia(skia::PathOp::XOR),
        "reverse_difference" => EngineOp::Skia(skia::PathOp::ReverseDifference),
        "add" => EngineOp::FloCurves(FloPathOp::Add),
        "flo_intersect" => EngineOp::FloCurves(FloPathOp::Intersect),
        "remove_interior" => EngineOp::FloCurves(FloPathOp::RemoveInterior),
        "remove_overlapping" => EngineOp::FloCurves(FloPathOp::RemoveOverlapping),
        "sub" => EngineOp::FloCurves(FloPathOp::Sub),
        s => panic!("flo_curves mode {} unavailable", s),
    };

    let mut path: glifparser::Glif<()> = glifparser::read(&fs::read_to_string(path_string).expect("Failed to read path file!"))
        .expect("glifparser couldn't parse input path glif. Invalid glif?");

    if let Some(ref outline) = path.outline.as_ref() {
        let mut final_output = match engine_op {
            EngineOp::Skia(pathop) => apply_skia(pathop, operand_string, outline),
            EngineOp::FloCurves(pathop) => apply_flo(pathop, operand_string, outline),
        };

        final_output.refigure_point_types();
        path.outline = Some(final_output);
    }
    glifparser::write_to_filename(&path, out_string).unwrap();
}
