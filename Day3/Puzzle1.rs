use std::fs;

fn main() {
    let file_path = "./input.txt";

    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let banks: Vec<&str> = input.split("\n").collect();
    let mut total_joltage = 0;
 
    // For each bank, find the first largest number, and then the second largest that comes after
    // combine, parse as int, and add to total
    for bank in banks {
        if bank.trim() == "" { continue; }
        println!("Bank: {}", bank);
 
        let largest = find_largest(&bank[..bank.len() - 1]);
        let largest_index = bank.find(&largest).unwrap();
        let second_largest = find_largest(&bank[largest_index+1..]);
        println!("Largest: {}", largest);
        println!("Second Largest after {}: {}", largest, second_largest); 

        let joltage = (largest + &second_largest).parse::<i32>().unwrap();
        println!("joltage: {}", joltage);
        total_joltage += joltage;
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
