use std::fs;

fn main() {
    let file_path = "./input.txt";

    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");
 
    //parse each number by row and column
    let mut lines: Vec<&str> = input.trim().split("\n").collect();    
    let numbers: Vec<Vec<i64>> = {
        let mut temp: Vec<Vec<i64>> = Vec::new();
        for (i, line) in lines.clone().into_iter().enumerate() {
            if i != lines.len() - 1 {
                let mut temp2: Vec<i64> = Vec::new();
                for slice in line.split_whitespace().into_iter() {
                    if slice.trim() != "" {
                        temp2.push(slice.parse::<i64>().unwrap());
                    }
                }
                temp.push(temp2);
            }
        }
        temp
    };
    let operations: Vec<&str> = lines.last().unwrap().split_whitespace().collect();
    let mut total = 0;
    let mut column_lengths: Vec<usize> = Vec::new();

    lines.pop(); //no longer need last line of operations

    //find the max length for each column
    for (i, row) in numbers.clone().into_iter().enumerate() {
        for (j, num) in row.into_iter().enumerate() {
            let len = num.to_string().len();
            if i == 0 {
                column_lengths.push(len);
            } else {
                if column_lengths[j] < len {
                    column_lengths[j] = len;
                } 
            }
        }
    }

    //println!("{:#?}", numbers);
    //println!("{:#?}", operations);
    //println!("{:#?}", column_lengths);

    // Loop over each column and construct the numbers from the smaller columns found within each
    for i in 0..column_lengths.len() {
        let mut constructed_numbers: Vec<String> = Vec::new();
        let offset = if i == 0 { 0 } else {
            let mut temp = 0;
            for j in 0..i {
                temp += column_lengths[j] + 1;
            }
            temp
        };

        //looping over smaller columns and pushing the next digit to the correct number
        for j in 0..column_lengths[i] {
            for (k, line) in lines.clone().into_iter().enumerate() {
                let index = j + offset;
                if index >= line.len() { continue; }
                let num = String::from(line[index..=index].trim());
                if k == 0 {
                    constructed_numbers.push(num);
                } else if num != "" {
                    if constructed_numbers.len() <= j {
                        constructed_numbers.push(num);
                    } else {
                        constructed_numbers[j].push_str(&num);
                
                    }
                }
            }
        }
        //println!("constructed_numbers: {:#?}", constructed_numbers);

        total += match operations[i] {
            "+" => {
                constructed_numbers.into_iter().map(|num| num.parse::<i64>().unwrap()).sum()
            },
            "*" => {
                constructed_numbers.into_iter().map(|num| num.parse::<i64>().unwrap()).product()
            },
            &_ => {
                0
            }
        };
    }


    println!("total: {}", total);

}
