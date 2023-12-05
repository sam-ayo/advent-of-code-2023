use std::io::Error;
use lib::input_array;


fn get_first_last_digits(input: Vec<String>) -> Result<u32, Error> {
    let result: Vec<u32> = input.iter().map(|line| {
        let line_arr: Vec<char> = line.chars().filter(|x| x.is_digit(10)).collect();
        let first = line_arr.first().unwrap().to_digit(10).unwrap();
        let last = line_arr.last().unwrap().to_digit(10).unwrap();
        return first * 10 + last;
    }).collect();

    let arr_sum: u32 = result.iter().sum();

    Ok(arr_sum)
}

fn main() {
    let input: Vec<String> = input_array("./src/inputs/input.txt").expect("");
    let digits: u32 = get_first_last_digits(input).expect("");
    println!("{:?}", digits);
}