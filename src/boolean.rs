pub use skia_safe as skia;
use std::fs;

use flo::bezier::path as flopath;
use flo_curves as flo;
use glifparser::outline::skia::{FromSkiaPath as _, ToSkiaPaths as _};
use MFEKmath::{Bezier, Piecewise};

type PwBez = Piecewise<Bezier>;

pub fn apply_flo<PD: glifparser::PointData>(pathop: FloPathOp, operand: Option<&str>, outline: &glifparser::Outline<PD>) -> glifparser::Outline<PD> {
    let pw: Piecewise<PwBez> = Piecewise::from(outline);
    let o_pw: Option<Piecewise<PwBez>> = {
        operand.map(|operand| {
            let operand: glifparser::Glif<()> = glifparser::read(&fs::read_to_string(operand).expect("Failed to read operand path file!"))
                .expect("glifparser couldn't parse operand path glif. Invalid glif?");
            Piecewise::from(&operand.outline.expect("No <outline> in operand glif"))
        })
    };

    let out = match pathop {
        FloPathOp::RemoveInterior => flopath::path_remove_interior_points::<PwBez, PwBez>(&pw.segs, 0.000001),
        FloPathOp::RemoveOverlapping => flopath::path_remove_overlapped_points::<PwBez, PwBez>(&pw.segs, 0.000001),
        FloPathOp::Intersect => flopath::path_intersect::<PwBez, PwBez, PwBez>(&pw.segs, &o_pw.expect("mode requires operand").segs, 0.000001),
        FloPathOp::Add => flopath::path_add::<PwBez, PwBez, PwBez>(&pw.segs, &o_pw.expect("mode requires operand").segs, 0.000001),
        FloPathOp::Sub => flopath::path_sub::<PwBez, PwBez, PwBez>(&pw.segs, &o_pw.expect("mode requires operand").segs, 0.000001),
    };

    Piecewise::new(out, None).to_outline()
}

pub fn apply_skia<PD: glifparser::PointData>(pathop: skia::PathOp, operand: Option<&str>, outline: &glifparser::Outline<PD>) -> glifparser::Outline<PD> {
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
pub enum FloPathOp {
    Add,
    Intersect,
    RemoveInterior,
    RemoveOverlapping,
    Sub,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EngineOp {
    Skia(skia::PathOp),
    FloCurves(FloPathOp),
}
