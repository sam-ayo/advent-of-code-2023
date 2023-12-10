use lib::get_input;

fn main () {
    let cards = [1,2,3,4,5,6];
    // let new_cards = [2,3,4,5,3,4,3,4,4,5,4,5,4,5,4,5,5,5,5,5,5,5,5,5];
    
    // let all_cards = vec![
    //     [vec![41, 48, 83, 86, 17], vec![83, 86, 6, 31, 17, 9, 48, 53]],
    //     [vec![13, 32, 20, 16, 61], vec![61, 30, 68, 82, 17, 32, 24, 19]],
    //     [vec![1, 21, 53, 59, 44],  vec![69, 82, 63, 72, 16, 21, 14, 1]], 
    //     [vec![41, 92, 73, 84, 69], vec![59, 84, 76, 51, 58, 5, 54, 83]],
    //     [vec![87, 83, 26, 28, 32], vec![88, 30, 70, 12, 93, 22, 82, 36]],
    //     [vec![31, 18, 13, 56, 72], vec![74, 77, 10, 23, 35, 67, 36, 11]]
    // ];
    
    let input = get_input("./src/input.txt").unwrap();
    let all_cards = all_cards_data_structure(input);
    let mut cards = Vec::new();
    for i in 1..all_cards.len()+1 {
        let card_no = i as i32;
        cards.push(card_no);
    }

    let mut processed: Vec<i32> = Vec::new();

    for card_no in 0..all_cards.len() {
        let mut processed_count = 0;
        for copy_card in &processed{
            if *copy_card == (card_no + 1)as i32{
                processed_count += 1;
            }
        }
        for other_card in &cards {
            if *other_card == (card_no + 1) as i32 {
                processed_count+=1;
            }
        }
        let card = &all_cards[card_no];
        let [ winning_numbers,  your_numbers] = card;
        let number_of_matching_numbers = get_number_of_matching_numbers(winning_numbers, your_numbers);

        let card_slice = &cards[card_no+1..( number_of_matching_numbers + card_no as i32 + 1) as usize];

        for _ in 0..processed_count {
            processed = processed.iter().chain(card_slice).map(|e| *e).collect();
        }

        if processed_count == 0 {
            processed = processed.iter().chain(card_slice).map(|e| *e).collect();
        }

        println!("For loop ran: {}", card_no+1)
    }
    
    let total_len = processed.len() + cards.len();
    println!("{:?}", total_len);
}

fn get_number_of_matching_numbers(winning_numbers: &Vec<i32>, your_numbers: &Vec<i32>) -> i32 {
    let mut count = 0;
    for number in winning_numbers {
        let your_number_match = your_numbers.iter().find(|e| *e == number);
        if let Some(_) = your_number_match {
            count += 1
        }
    }
    return count;
}

fn all_cards_data_structure (input: String) -> Vec<[Vec<i32>; 2]>  {
    let mut all_cards: Vec<[Vec<i32>; 2]> = Vec::new();
    for line in input.lines(){
        let card_vec: Vec<&str> = line.split(":").collect();
        let numbers = card_vec[1];
        
        let card: Vec<&str> = numbers.split("|").into_iter().map(|e| e.trim()).collect();
        
        let winning_numbers_str =  card[0];
        let your_numbers_str = card[1];

        let winning_numbers: Vec<i32> = winning_numbers_str.split(" ").filter(|e| *e !=  ""  ).map(|e| e.parse().unwrap()).collect();
        let your_numbers: Vec<i32> = your_numbers_str.split(" ").filter(|e| *e !=  ""  ).map(|e| e.parse().unwrap()).collect();
        
        // println!("{:?}", winning_numbers);
        // println!("{:?}", your_numbers);
        
        let card_array = [winning_numbers, your_numbers];
        all_cards.push(card_array);
    }
    all_cards
} 