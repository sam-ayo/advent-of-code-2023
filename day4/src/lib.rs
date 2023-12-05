use std::fs::File;
use std::io::{prelude::*, Error};

pub fn get_input (file_path: &str) -> Result<String, Error > {
    let mut file = File::open(file_path).unwrap();
    let mut buffer = String::new();
    let _ = file.read_to_string(&mut buffer);
    Ok(buffer)
}

struct Card {
    winning_numbers: Vec<u16>,
    your_numbers: Vec<u16>
}
struct Games {
    games: Vec<Card>,
}

pub fn input_to_data_structure (input: String)  {
    let mut total_sum = 0;
    for line in input.lines(){
        let card_vec: Vec<&str> = line.split(":").collect();
        let card_no = card_vec[0];
        let numbers = card_vec[1];
        
        let card: Vec<&str> = numbers.split("|").into_iter().map(|e| e.trim()).collect();
        
        let winning_numbers_str =  card[0];
        let your_numbers_str = card[1];

        let winning_numbers: Vec<u32> = winning_numbers_str.split(" ").filter(|e| *e !=  ""  ).map(|e| e.parse().unwrap()).collect();
        let your_numbers: Vec<u32> = your_numbers_str.split(" ").filter(|e| *e !=  ""  ).map(|e| e.parse().unwrap()).collect();
        
        
        let mut count = 0;
        let mut product = 0;

        winning_numbers.into_iter().for_each(|number|  {
            let mut your_numbers_clone = your_numbers.clone();
            your_numbers_clone.sort();
            let answer = your_numbers_clone.binary_search(&number);

            let is_winning = match answer {
                Ok(_) => true,
                Err(_) => false,
            };
            
            if is_winning {
                if count == 0 {
                    count += 1;
                    product += 1
                }else {
                    product *= 2;
                }
            }
        });
        
        total_sum += product;
    }
    println!("{total_sum}");

}



