use std::{fs::File, io::Error};
use std::io::prelude::*;

 fn get_input(file_path: &str) -> Result<String, Error>{
    let mut f = File::open(file_path)?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;
    Ok(buffer)
}

 fn transform_input(full_string: String) -> Result<Vec<String>, Error>{
    let array = full_string.lines().map(String::from).collect();
    Ok(array)
}

pub fn input_array(file_path: &str) -> Result<Vec<String>, Error > {
    let input_string = get_input(file_path)?;
    let array = transform_input(input_string)?;
    Ok(array)
}
