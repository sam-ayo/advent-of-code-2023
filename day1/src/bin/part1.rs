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
    
    let array = [1,2,3,4];
    
    let new_copy_array = array.clone();

    Ok(arr_sum)
        // string = "1, 2, 3, 4" delimeter = ","
        // StrSplit::new(string, delimeter).map(x x.todigit()).sum();
}

fn main() {
    let input: Vec<String> = input_array("./src/inputs/input.txt").expect("");
    let digits: u32 = get_first_last_digits(input).expect("");
    println!("{:?}", digits);
}