use std::{path::Path, fs::File, io::Read};


pub fn load_text_input_from_file<P: AsRef<Path>>(path: P) -> String {
    load_text_input(File::open(path).unwrap())
}

pub fn load_text_input<R: Read>(mut input: R) -> String {
    let mut buffer = String::new();
    input.read_to_string(&mut buffer).unwrap();
    buffer
}
