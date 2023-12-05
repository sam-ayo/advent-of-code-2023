use lib::{get_input, input_to_data_structure};

fn main() {
    let file_path = "./src/input.txt";
    let input = get_input(file_path).unwrap();
    input_to_data_structure(input);
}