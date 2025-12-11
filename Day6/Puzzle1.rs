use std::fs;

fn main() {
    let file_path = "./input.txt";

    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");
    
    let lines: Vec<&str> = input.trim().split("\n").collect();    
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
    //println!("{:#?}", numbers); 
    //println!("{:#?}", operations);

    for i in 0..operations.len() {
        let mut running_total = 0;
        for (j, group) in numbers.clone().into_iter().enumerate() {
            if j == 0 { running_total += group[i]; }
            else {
                match operations[i] {
                    "*" => {
                        running_total *= group[i];
                    },
                    "+" => {
                        running_total += group[i];
                    },
                    &_ => {
                        //do nothing
                    }
                }
            }
        }
        total += running_total;
    }
    println!("Total is {}", total);
}
