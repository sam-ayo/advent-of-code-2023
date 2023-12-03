use std::collections::HashMap;
use std::{fs::File, io::BufRead};
use std::io::{prelude::*, Error};


pub fn get_input (file_path: &str) -> Result<String, Error>{
    let mut file = File::open(file_path).unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();
    Ok(buffer)
}

pub fn input_to_data_structure(input: String) {
    let input_arr: Vec<String>  = input.lines().map(|x| x.to_string()).collect();

    let game_with_subset_lists: Vec<Vec<String>> = input_arr.into_iter().map(|game| {
        let arr: Vec<String> = game.split(":").map(|x| x.to_string()).collect() ;
        return arr;
    }).collect();
    
    let result: Vec<HashMap<String, Vec<String>>> =  game_with_subset_lists.into_iter().map(|game_instance| {
        let subsets_list = &game_instance[1];
        let each_game: Vec<String> = subsets_list.split(";").map(|x| x.to_string()).collect();
        HashMap::from([
            (game_instance[0].to_owned(), each_game)
        ])
    }).collect();
    
    let next_result = result.into_iter().for_each(|game_instance| {
        let values = game_instance.values().for_each(|value| {
            let answer = value.to_owned();
            println!("{:?}", answer);
        }
    );
        println!()
    });
    

    
}