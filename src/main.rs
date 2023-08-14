mod coordinates;
mod file;

use clap::Parser;
use coordinates::{get_epsg_param, transform_coordinates};
use serde_json::{from_str, to_string_pretty, Value};

/// Transform coordinates CRS
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Coordinates
    #[arg(short, long)]
    coordinates: Option<String>,

    /// File name
    #[arg(short, long)]
    file: Option<String>,

    /// Input CRS
    #[arg(short, long)]
    input_crs: u16,

    /// Output CRS
    #[arg(short, long)]
    output_crs: u16,
}

fn get_epsg_params(args: &Args) -> (String, String) {
    (
        get_epsg_param(args.input_crs),
        get_epsg_param(args.output_crs),
    )
}

fn transform_inline_coordinates(args: &Args) {
    let coordinates_string = args
        .coordinates
        .as_ref()
        .expect("--coordinates is not valid!");

    let coordinates: Value = from_str(&coordinates_string).expect("Couldn't parse JSON");
    let (input_crs, output_crs) = get_epsg_params(&args);

    let transformed_coordinates = transform_coordinates(coordinates, &input_crs, &output_crs);

    println!(
        "{:}",
        to_string_pretty(&transformed_coordinates).expect("Couldn't stringify geometry!")
    );
}

fn transform_file_coordinates(args: &Args) {
    let file_name = args.file.as_ref().expect("--file is not valid!");
    let coordinates = file::read_file_to_json(&file_name);
    let (input_crs, output_crs) = get_epsg_params(&args);

    let transformed_coordinates = transform_coordinates(coordinates, &input_crs, &output_crs);

    println!(
        "{:}",
        to_string_pretty(&transformed_coordinates).expect("Couldn't stringify geometry!")
    );
}

fn main() {
    let args = Args::parse();

    if args.coordinates.is_some() {
        transform_inline_coordinates(&args);
    } else if args.file.is_some() {
        transform_file_coordinates(&args);
    } else {
        panic!("There is no input data! Use --coordinates or --file, please.");
    }
}
