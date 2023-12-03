use lib::{get_input, input_to_data_structure, TotalCubes, get_possible_games, get_sum_possible_game_id};

fn main () {
   let input = get_input("./src/inputs/part1.txt").unwrap(); 

   let input_data_strucuture = input_to_data_structure(input);

   let total_cubes = TotalCubes::new(12, 13, 14);
   let possible_games = get_possible_games(input_data_strucuture, total_cubes);

   let sum_possible_game_id = get_sum_possible_game_id(possible_games);
   
   println!("{}", sum_possible_game_id);

}

