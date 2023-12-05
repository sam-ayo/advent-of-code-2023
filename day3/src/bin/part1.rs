use lib::{get_input, input_to_data_structure};

fn main () {
    let path = "src/inputs/input.txt";
    let input = get_input(path).unwrap();
    let parsed_input = input_to_data_structure(input);
}