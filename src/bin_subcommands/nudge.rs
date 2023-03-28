use std::fs;
use std::collections::HashSet;

use super::*;

use glifparser::{Handle, Glif, Point};

use crate::validators::arg_index_or_range;

pub fn clap_app() -> clap::App<'static> {
    App::new("NUDGE")
            .setting(AppSettings::DeriveDisplayOrder)
            .setting(AppSettings::AllowNegativeNumbers)
            .about("Moves points, contours or all points in a glyph in UFO .glif format.")
            .version("0.1.0")
            //                                                                 (reviewed by a human, see MFEK_GPT_POLICY.md)
            .author("Fredrick R. Brennan <copypaste@kittens.ph>, MFEK Authors, and GPT-4")
            .arg(Arg::new("x")
                .long("x")
                .short('x')
                .takes_value(true)
                .default_value("0")
                .help("X-axis movement"))
            .arg(Arg::new("y")
                .long("y")
                .short('y')
                .takes_value(true)
                .default_value("0")
                .help("Y-axis movement"))
            .arg(Arg::new("input")
                .long("input")
                .short('i')
                .takes_value(true)
                .required(true)
                .help("The path to the input glif file."))
            .arg(Arg::new("output")
                .long("output")
                .short('o')
                .required(true)
                .takes_value(true)
                .help("The path to the output glif file."))
            .arg(Arg::new("contour")
                .long("contour")
                .short('c')
                .takes_value(true)
                .multiple_values(true)
                .use_delimiter(true)
                .validator(arg_index_or_range)
                .help("Indices of contours to move."))
            .arg(Arg::new("point")
                .long("point")
                .short('p')
                .takes_value(true)
                .multiple_values(true)
                .validator(arg_index_or_range)
                .use_delimiter(true)
                .help("Indices of points to move."))
}

pub fn cli(matches: &ArgMatches) {
    let x: f32 = matches.value_of("x").unwrap().parse().expect("Invalid x value");
    let y: f32 = matches.value_of("y").unwrap().parse().expect("Invalid y value");
    let input_string = matches.value_of("input").unwrap();
    let output_string = matches.value_of("output").unwrap();
    let contour_indices = matches.values_of("contour")
        .map(|values| values.map(|val| val.parse().expect("Invalid contour index")).collect::<Vec<usize>>())
        .unwrap_or_else(Vec::new);
    let point_indices = matches.values_of("point")
        .map(|values| values.map(|val| val.parse().expect("Invalid point index")).collect::<Vec<usize>>())
        .unwrap_or_else(Vec::new);

    let mut path: Glif<()> = glifparser::read(&fs::read_to_string(input_string).expect("Failed to read input file!"))
        .expect("glifparser couldn't parse input path glif. Invalid glif?");

    if let Some(ref mut outline) = path.outline.as_mut() {
        let mut points_to_move = HashSet::new();

        if !contour_indices.is_empty() {
            for contour_idx in contour_indices.into_iter() {
                let contour = &outline[contour_idx];
                for (point_idx, _) in contour.iter().enumerate() {
                    points_to_move.insert((contour_idx, point_idx));
                }
            }
        }

        if !point_indices.is_empty() {
            for point_idx in point_indices.into_iter() {
                let contour_idx = outline.into_iter()
                    .position(|contour| contour.into_iter().any(|point| point.name == Some(point_idx.to_string())))
                    .expect("Point not found in any contour");

                points_to_move.insert((contour_idx, point_idx));
            }
        }

        let move_point = |point: &mut Point<()>| {
            point.x += x;
            point.y += y;
            if let Handle::At(ax, ay) = point.a {
                point.a = Handle::At(ax + x, ay + y);
            }
            if let Handle::At(bx, by) = point.b {
                point.b = Handle::At(bx + x, by + y);
            }
        };

        if !points_to_move.is_empty() {
            for (contour_idx, point_idx) in points_to_move {
                let point = &mut outline[contour_idx][point_idx];
                move_point(point);
            }
        } else {
            for contour in outline.iter_mut() {
                for point in &mut contour.iter_mut() {
                    move_point(point);
                }
            }
        }
    } else {
        return;
    }

    glifparser::write_to_filename(&path, output_string).unwrap();
}
