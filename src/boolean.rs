use clap::{App, AppSettings, Arg, ArgMatches};
use skia_safe as skia;
use std::fs;

use flo::bezier::path as flopath;
use flo_curves as flo;
use glifparser::outline::skia::{FromSkiaPath as _, ToSkiaPaths as _};
use glifparser::outline::RefigurePointTypes as _;
use MFEKmath::{Bezier, Piecewise};

type PwBez = Piecewise<Bezier>;

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
                .possible_values(&["difference", "intersect", "union", "xor", "reverse_difference", "add", "intersect", "remove_interior", "remove_overlapping", "sub"])
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

fn apply_flo<PD: glifparser::PointData>(pathop: FloPathOp, operand: Option<&str>, outline: &glifparser::Outline<PD>) -> glifparser::Outline<()> {
    let pw: Piecewise<PwBez> = Piecewise::from(outline);
    let o_pw: Option<Piecewise<PwBez>> = {
        operand.map(|operand| {
            let operand: glifparser::Glif<()> = glifparser::read(&fs::read_to_string(operand).expect("Failed to read operand path file!"))
                .expect("glifparser couldn't parse operand path glif. Invalid glif?");
            Piecewise::from(&operand.outline.expect("No <outline> in operand glif"))
        })
    };

    let out = match pathop {
        FloPathOp::RemoveInterior => flopath::path_remove_interior_points::<PwBez, PwBez>(&pw.segs, 1.),
        FloPathOp::RemoveOverlapping => flopath::path_remove_overlapped_points::<PwBez, PwBez>(&pw.segs, 1.),
        FloPathOp::Intersect => flopath::path_intersect::<PwBez, PwBez, PwBez>(&pw.segs, &o_pw.expect("mode requires operand").segs, 1.),
        FloPathOp::Add => flopath::path_add::<PwBez, PwBez, PwBez>(&pw.segs, &o_pw.expect("mode requires operand").segs, 1.),
        FloPathOp::Sub => flopath::path_sub::<PwBez, PwBez, PwBez>(&pw.segs, &o_pw.expect("mode requires operand").segs, 1.),
    };

    Piecewise::new(out, None).to_outline()
}

fn apply_skia(pathop: skia::PathOp, operand: Option<&str>, outline: &glifparser::Outline<()>) -> glifparser::Outline<()> {
    let skp = outline.to_skia_paths(None).combined();
    let mut final_skpath;

    let operand = operand.map(|oper| {
        glifparser::read::<()>(&fs::read_to_string(oper).expect("Failed to read operand path file!"))
            .expect("glifparser couldn't parse operand path glif. Invalid glif?")
            .outline
            .expect("no <outline> in glif")
            .to_skia_paths(None)
            .combined()
    });

    if pathop == skia::PathOp::Union && operand.is_none() {
        final_skpath = skp.op(&skp, pathop).unwrap();
        final_skpath = final_skpath.op(&skp, pathop).unwrap();
    } else {
        let operand = operand.unwrap();
        final_skpath = skp.op(&operand, pathop).unwrap();
    }

    final_skpath = final_skpath.as_winding().unwrap();

    glifparser::Outline::from_skia_path(&final_skpath)
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum FloPathOp {
    Add,
    Intersect,
    RemoveInterior,
    RemoveOverlapping,
    Sub,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum EngineOp {
    Skia(skia::PathOp),
    FloCurves(FloPathOp),
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
