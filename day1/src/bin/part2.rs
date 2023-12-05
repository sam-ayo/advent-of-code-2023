use std::{collections::HashMap, fmt::Error};

use lib::input_array;
fn main () {
    let path = "./src/inputs/input.txt";
    let arr = input_array(path).unwrap();
    let words_to_number_map = [
        ("one", 1), 
        ("two", 2),
        ("three", 3),
        ("four", 4), 
        ("five", 5), 
        ("six", 6), 
        ("seven", 7), 
        ("eight", 8), 
        ("nine", 9)
    ];
    let map = HashMap::from(words_to_number_map);
    
    let total_sum =  map_words_to_number(arr, map).unwrap();
    
    println!("{total_sum}");

}
fn get_number(index: usize, count: usize, line: &str, words_to_number_map: &HashMap<&str, i32>) -> i32 {
    let word = &line[index..index+count];
    match words_to_number_map.get(word) {
        Some(num) =>  *num,
        None => 0
    }
}

fn map_words_to_number(arr: Vec<String>, words_to_number_map: HashMap<&str, i32>) -> Result<u32, Error> {
    
    let mappings = [
        ('o', vec![3]),
        ('t', vec![3, 5]),
        ('f', vec![4]),
        ('s', vec![3, 5]),
        ('e', vec![5]),
        ('n', vec![4]),
    ];

    let important_char: HashMap<char, Vec<u8>> = HashMap::from(mappings);
    

    
    let new_arr: Vec<String> = arr.iter().map(|line| {
        return line.chars().enumerate().map(|(index, char)| {
            let mut number_string = String::new();
            let words_count = important_char.get(&char);
            if char.is_digit(10) {
                number_string = format!("{}{}", number_string, char); 
            }
            if words_count.is_some() {
                let count = words_count.unwrap();
                if count.len()  == 2 {
                    let first = count[0] as usize;
                    let mut number = 0;
                    if line.len() - index >= first {
                        number = get_number(index, first, line, &words_to_number_map);
                    }

                    if number == 0 {
                        let second = count[1] as usize;
                        if line.len() - index >= second {
                            number = get_number(index, second, line, &words_to_number_map);
                        }
                    }
                    let string = number.to_string();
                    number_string = format!("{}{}", number_string, string);
                }else {
                    let first = count[0] as usize;
                    let mut number = 0;
                    if line.len() - index >= first {
                        number = get_number(index, first, line, &words_to_number_map);
                    }
                    let string = number.to_string();
                    number_string = format!("{}{}", number_string, string);
                }
            }
            return number_string;
        }).collect();
    }).collect();
    
    let arr_without_zero: Vec<String> = new_arr.into_iter().map(|number_string| {
        return number_string.chars().filter(|char| {
            return *char != '0';
        }).collect();
    }).collect();
    
    let two_digits: Vec<u32> = arr_without_zero.into_iter().map(|number_string| {
        let arr_of_chars: Vec<char> = number_string.chars().collect();
        let first = arr_of_chars.first().unwrap().to_digit(10).unwrap();
        let last = arr_of_chars.last().unwrap().to_digit(10).unwrap();
        return first * 10 + last;
    }).collect();
    
    let total_sum: u32 = two_digits.into_iter().sum();
    
    Ok(total_sum)
}
