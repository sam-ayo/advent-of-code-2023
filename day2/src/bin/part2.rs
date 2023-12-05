use lib::{input_to_data_structure, get_input, get_minimum_set_of_cubes, sum_power_of_minimum_set_of_cubes};

fn main () {
    let input = get_input("./src/inputs/input.txt").unwrap(); 

    let input_data_strucuture = input_to_data_structure(input);    
    
    let minimum_set =  get_minimum_set_of_cubes(input_data_strucuture);
    
    let result = sum_power_of_minimum_set_of_cubes(minimum_set);
    println!("{}", result);
}