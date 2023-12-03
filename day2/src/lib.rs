use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, Error};


pub fn get_input (file_path: &str) -> Result<String, Error>{
    let mut file = File::open(file_path).unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();
    Ok(buffer)
}

pub fn input_to_data_structure(input: String) -> HashMap<String, Vec<HashMap<String, u32>>> {
    let input_arr: Vec<String>  = input.lines().map(|x| x.to_string()).collect();

    let game_with_subset_lists: Vec<Vec<String>> = input_arr.into_iter().map(|game| {
        let arr: Vec<String> = game.split(":").map(|x| x.to_string()).collect() ;
        return arr;
    }).collect();
    
    
    let mut hashmap: HashMap<String, Vec<String>> = HashMap::new();

     game_with_subset_lists.into_iter().for_each(|game_instance| {
        let subsets_list = &game_instance[1];
        let each_game: Vec<String> = subsets_list.split(";").map(|x| x.to_string()).collect();
        hashmap.insert(game_instance[0].to_owned(), each_game);
    });

     
    let mut clean_hashmap: HashMap<String, Vec<HashMap<String, u32>>> = HashMap::new();
     hashmap.keys().for_each(|k| {
        let subset_list: Vec<String> = hashmap.get(k).unwrap().into_iter().map(|s| s.trim().to_string()).collect();
        let mut all_array : Vec<HashMap<String, u32>>= Vec::new();
        subset_list.into_iter().for_each(|e| {
            let mut pick_map: HashMap<String, u32> = HashMap::new();
            let each_pick: Vec<String> = e.split(",").map(|e| e.trim().to_string()).collect();
            
            each_pick.into_iter().for_each(|e|{
                let split: Vec<String> = e.split(" ").map(|e| e.trim().to_string()).collect();
                let color = split[1].clone();
                let number: u32 = split[0].parse().unwrap();
                pick_map.insert(color, number);
            });
            all_array.push(pick_map);
        });
        clean_hashmap.insert(k.to_owned(), all_array);
     });
     
     clean_hashmap
}

pub struct TotalCubes {
    red: u32,
    green: u32,
    blue: u32,
}

impl TotalCubes {
    pub fn new (red: u32, green: u32, blue: u32) -> Self{
        TotalCubes { red, green, blue}
    }
}

pub fn get_possible_games (clean_hashmap: HashMap<String, Vec<HashMap<String, u32>>>, total_cubes: TotalCubes ) -> HashMap<String, Vec<HashMap<String, u32>>> {
    let mut possible_games_hashmap: HashMap<String, Vec<HashMap<String, u32>>> = HashMap::new();
    clean_hashmap.keys().for_each(|k| {
        let game = clean_hashmap.get(k).unwrap();
        
        let mut new_game: Vec<HashMap<String, u32>> = Vec::new();
        game.into_iter().for_each(|subset| {
            let red = match subset.get("red") {
                Some(r) => r.to_owned(),
                None => 0
            };

            let green = match subset.get("green") {
                Some(g) => g.to_owned(),
                None => 0
            };

            let blue = match subset.get("blue") {
                Some(b) => b.to_owned(),
                None => 0
            };
            
            let expected_red = total_cubes.red;
            let expected_green = total_cubes.green;
            let expected_blue =total_cubes.blue;
            
            let is_subset_possible= red <= expected_red && green <= expected_green && blue <= expected_blue; 
            
            if is_subset_possible {
                new_game.push(subset.to_owned());
            }
        });

        if new_game.len() == game.len() {
            let key = k.to_string();
            possible_games_hashmap.insert(key, game.to_owned());
        }
    });
    possible_games_hashmap
}

pub fn get_sum_possible_game_id (possible_game_hasmap:HashMap<String, Vec<HashMap<String, u32>>>) -> u32 {
    let mut total_sum = 0;
     possible_game_hasmap.keys().for_each(|game_string| {
        let game_string_with_id: Vec<String> = game_string.split(" ").map(|e| e.trim().to_string()).collect();
        let id: u32 = game_string_with_id[1].parse().unwrap();
        total_sum = total_sum + id;
    });
    total_sum
}