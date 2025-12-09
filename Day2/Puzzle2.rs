use std::fs;

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
            let mut search_len = 1;
            //println!("Current num: {}", digits);

            while search_len <= len / 2 {
                if len % search_len != 0 {
                    //println!("search_len does not divide evenly");
                    search_len += 1;
                    continue;
                }
 
                let repeating_num = &digits[..search_len];
                let mut prev_index = search_len;
                let mut current_index = search_len + search_len;
                let mut is_repeating = true;
                //println!("Checking for repeating {}", repeating_num);
                while current_index <= len {
                    let tmp = &digits[prev_index..current_index];
                    if tmp != repeating_num {
                        is_repeating = false;
                        break;
                    }
                    prev_index = current_index;
                    current_index = current_index + search_len;
                }
                if is_repeating {
                    total += i;
                    println!("Invalid ID: {}", i);
                    break;
                }

                search_len += 1;
            }

            i += 1;
        }
    }

    println!("Total of all invalid id values: {}", total);
}
