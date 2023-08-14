use std::{fs::File, io::Read};

use serde_json::{from_str, Value};

pub fn read_file_content(file_name: &String) -> String {
    let mut file =
        File::open(file_name).expect(format!("File {0} doesn't exist!", file_name).as_str());

    let mut file_content = String::new();
    file.read_to_string(&mut file_content)
        .expect("Couldn't read the file content!");

    file_content
}

pub fn read_file_to_json(file_name: &String) -> Value {
    let json_string = read_file_content(file_name);

    from_str(&json_string).expect("Couldn't deserialize the file!")
}
