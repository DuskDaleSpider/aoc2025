use std::fs;
use std::convert::TryInto;

#[derive(Debug)]
struct Range {
    start: i64,
    end: i64
}

fn main() {
    let file_path = "./input.txt";

    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let input_ranges: Vec<&str> = input.split(",").collect();
    let mut ranges: Vec<Range> = Vec::new();
    let mut total: i64 = 0;

    for mut input_range in input_ranges {
        input_range = input_range.trim();
        let dash_index = input_range.find("-").unwrap();
        ranges.push(Range {
            start: input_range[..dash_index].parse::<i64>().unwrap(),
            end: input_range[dash_index+1..].parse::<i64>().unwrap()
        });
    }

    for range in ranges {
        println!("{:#?}", range);

        let mut i = range.start;
        while i <= range.end {
            let digits = i.to_string(); 
            let len = digits.len();

            //continue or break loop if digits can't be split evenly
            if len % 2 != 0 {
                println!("Digit has uneven length: {}", digits);
                let next_valid = 10_i64.pow(len.try_into().unwrap());
                println!("Next Valid: {}", next_valid);
                if next_valid > range.end {
                    break;
                } else {
                    i = next_valid;
                    continue;
                }
            }

            let first_half = digits[..len/2].parse::<i32>().unwrap();
            let second_half = digits[len/2..].parse::<i32>().unwrap();

            //invalid id
            if first_half == second_half {
                println!("Invalid ID: {}", i);
                total += i;
            }
            i += 1;
        }
    }

    println!("Total of all invalid id values: {}", total);
}
