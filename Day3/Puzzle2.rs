use std::fs;

fn main() {
    let file_path = "./input.txt";

    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let banks: Vec<&str> = input.split("\n").collect();
    let mut total_joltage: i64 = 0;
 
    // for each bank, find the first largest possible digit, then repeatedly find the next largest possible
    // digit that comes after the prev, concat to currently known digits until we have or can
    // construct one with 12 digits
    // once 12 largest possbile digits found, parse as int and add to total
    for bank in banks {
        if bank.trim() == "" { continue; }
        println!("Bank: {}", bank);
        let mut digits_left = 11;
        let mut digits = String::new(); 
        let mut current_index = 0;
        let mut prev_index = 0;
        let len = bank.len();

        loop {
            let slice = String::from(&bank[current_index..len - digits_left]);
            let largest = find_largest(&slice);
            current_index = slice.find(&largest).unwrap() + prev_index;

            //found largest digit in last possible index
            //can construct the rest of the number
            if len - digits_left == current_index {
                digits = digits.to_owned() + &bank[current_index..];
                break;
            }else{
                digits += &largest;
            }

            if digits_left == 0 { break; }
            else { 
                digits_left -= 1;
                current_index += 1; 
                prev_index = current_index;
            }
        }

        println!("Digits: {}", digits);
        total_joltage += digits.parse::<i64>().unwrap();
    }

    println!("Total joltage: {}", total_joltage);
}

fn find_largest(slice: &str) -> String {
    let mut largest = '0';
    for digit in slice.chars() {
        if digit > largest {
            largest = digit;
        }
    }
    return largest.to_string();
}
