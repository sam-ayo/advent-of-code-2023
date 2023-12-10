use std::fs::File;
use std::io::{prelude::*, Error};

pub fn get_input (file_path: &str) -> Result<String, Error>{
    let mut file = File::open(file_path).unwrap();
    let mut buffer = String::new();
    let _ = file.read_to_string(&mut buffer);
    Ok(buffer)
}

pub fn input_to_data_structure(input: String) -> Vec<Vec<char>>{
    let input_vec: Vec<Vec<char>> = input.lines().map(|line| {
        let line_chars: Vec<char> = line.chars().collect();
        return line_chars
    }).collect();
    
    input_vec
}


pub fn get_part_numbers (parsed_input: Vec<Vec<char>>) -> Vec<u32> {
    let row_len = parsed_input[0].len();
    let column_len = parsed_input.len();
    for i in 0..column_len {
        for j in 0..row_len {
            let curr_node = parsed_input[i][j];
            let Neighbours{top_left, top, top_right,
                 left, right,bottom_left
                 , bottom, bottom_right } =  get_all_neighbours(i, j, &parsed_input);

        }
    }
    vec![32]
}

pub struct Neighbours {
    top_left: char,
    top: char,
    top_right: char,
    left: char,
    right: char,
    bottom_left: char,
    bottom: char,
    bottom_right: char,
}
impl Neighbours {
    fn new(
    top_left: char,
    top: char,
    top_right: char,
    left: char,
    right: char,
    bottom_left: char,
    bottom: char,
    bottom_right: char,
) -> Self {
        Neighbours { top_left, top, top_right, left, right, bottom_left , bottom, bottom_right }
    } 
}

pub fn get_all_neighbours(i: usize, j: usize, parsed_input: &Vec<Vec<char>>) -> Neighbours {
    let top_left = parsed_input[i-1][j-1];
    let top = parsed_input[i-1][j];
    let top_right =  parsed_input[i-1][j+1];
    let left = parsed_input[i][j-1];
    let right = parsed_input[i][j+1];
    let bottom_left = parsed_input[i+1][j-1];
    let bottom = parsed_input[i+1][j];
    let bottom_right = parsed_input[i+1][j+1];
    
    Neighbours::new(top_left, top, top_right, left, right, bottom_left, bottom, bottom_right)

}

pub fn is_neighbour_valid (i:usize , j: usize, parsed_input: &Vec<Vec<char>>) -> bool {
    let row_length = parsed_input[0].len();
    let column_length  = parsed_input.len();
    if i <= row_length-1 && i >= 0 && j <= column_length-1 && j >= 0{
        return true;
    }
    false
}