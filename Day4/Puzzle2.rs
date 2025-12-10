use std::fs;
use std::convert::TryInto;

fn main() {
    let file_path = "./input.txt";

    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut rows: Vec<&str> = input.split("\n").collect();
    let num_rows:isize = (rows.len() - 1).try_into().unwrap(); //split above is adding an extra row
    let num_cols:isize = rows[0].len().try_into().unwrap();
    let paper_char = "@";
    let mut total = 0;
    let mut num_removed = 1;

    while num_removed > 0 {
        num_removed = 0;
        for x in 0isize..num_rows {
            let mut temp = String::from(rows[x as usize]);
            for y in 0isize..num_cols {
                if temp[y as usize..=y as usize] == *paper_char {
                    let mut available_areas = 0;
                    
                    for x2 in -1isize..=1 {
                        let x_index = x - x2;
                        if x_index < 0 || x_index >= num_rows {
                            available_areas += 3;
                            continue;
                        }
                        for y2 in -1isize..=1 {
                            let y_index = y - y2;
                            if y_index < 0 || y_index >= num_cols {
                                available_areas += 1;
                                continue;
                            }
                            let space = &temp[y_index as usize..=y_index as usize];
                            if space == "." {
                                available_areas += 1;
                            }
                        }
                    }

                    if available_areas >  4 {
                        temp.replace_range(y as usize..=y as usize, ".");
                        total += 1;
                        num_removed += 1;
                        print!("X");
                    } else {
                        print!("{}", &rows[x as usize][y as usize..=y as usize]);
                    }
                } else {
                    print!("{}", &rows[x as usize][y as usize..=y as usize]);
                }
            }
            rows[x as usize] = &temp;
        }

        println!("\n============= Num Removed: {} ===============", num_removed);
    }

    println!("Total rolls that are accessable: {}", total);
}
