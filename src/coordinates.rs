use proj::Proj;
use serde_json::{json, Value};

fn iterate_through_coords(coordinates: &mut Vec<Value>, input_crs: &str, output_crs: &str) {
    for item in coordinates {
        if is_point_array(&item) {
            let expect_error = "Couldn't parse coordinates to f64!";
            let lng = item[0].as_f64().expect(expect_error);
            let lat = item[1].as_f64().expect(expect_error);

            let transformed_point = transform_point_coordinates((lat, lng), input_crs, output_crs);
            let new_coordinates = vec![json!([transformed_point.0, transformed_point.1])];
            *item = Value::Array(new_coordinates);
        } else {
            if let Value::Array(item) = item {
                iterate_through_coords(item, input_crs, output_crs);
            } else {
                panic!("Value is not an array!");
            }
        }
    }
}

fn is_point_array(coordinates: &Value) -> bool {
    coordinates[0].is_f64() && coordinates[1].is_f64()
}

fn transform_point_coordinates(
    coordinates: (f64, f64),
    input_crs: &str,
    output_crs: &str,
) -> (f64, f64) {
    let proj_transform =
        Proj::new_known_crs(input_crs, output_crs, None).expect("Projection error!");
    
    proj_transform
        .convert(coordinates)
        .expect("Couldn't transform the coordinates.")
}

pub fn get_epsg_param(epsg_code: u16) -> String {
    format!("EPSG:{}", epsg_code)
}

pub fn transform_coordinates(coordinates: Value, input_crs: &str, output_crs: &str) -> Vec<Value> {
    if let Value::Array(coordinates) = coordinates {
        let mut transformed_coordinates = coordinates;

        iterate_through_coords(&mut transformed_coordinates, input_crs, output_crs);

        transformed_coordinates
    } else {
        panic!("Value is not an array!");
    }
}
