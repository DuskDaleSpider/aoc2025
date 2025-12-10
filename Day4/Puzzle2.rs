use std::fs;

fn main() {
    let file_path = "./input.txt";

    let input: Vec<u8> = fs::read(file_path).expect("Should have been able to read the file");
    let mut rows: Vec<Vec<u8>> = {
        let mut temp: Vec<Vec<u8>> = Vec::new();
        for row in input.split(|space| *space == b'\n') {
           if row.len() != 0 { temp.push(Vec::from(row)); } 
        }
        temp
    };
    let roll_char = '@';
    let mut total = 0;
    let mut num_removed = 1;
    let mut num_iter = 1;

    while num_removed > 0 {
        num_removed = 0;
        println!("========== Step {} =========", num_iter);
        for (x, row) in rows.clone().into_iter().enumerate() {
            for (y, space) in row.clone().into_iter().enumerate() {
                if space == roll_char as u8 {
                    let mut available_areas = 0;
                    
                    let start_offset = if x == 0 {
                        available_areas += 3;
                        0
                    } else { 1 };
                    let end_offset = if x == rows.len() - 1 {
                        available_areas += 3;
                        0
                    } else { 1 };

                    for x2 in (x - start_offset)..=(x + end_offset) {
                        let start_offset = if y == 0 {
                            available_areas += 1;
                            0
                        } else { 1 };
                        let end_offset = if y == row.len() - 1 {
                            available_areas += 1;
                            0
                        } else { 1 };

                        for y2 in (y - start_offset)..=(y + end_offset) {
                            if rows[x2][y2] == b'.' {
                                available_areas += 1;
                            }
                        }
                    }

                    if available_areas > 4 {
                        total += 1;
                        num_removed += 1;
                        rows[x][y] = b'.';
                        print!("X");
                    } else {
                        print!("{}", rows[x][y] as char);
                    }

                } else {
                    print!("{}", space as char);
                }

                //print!("{}", space as char);
            }
            println!("");
        }
        println!("Removed {} rolls", num_removed);
        println!("");
        num_iter += 1;
    }

    let num_rows = rows.len();
    let num_cols = rows[0].len();

    println!("Grid size: {} x {}", num_rows, num_cols);
    println!("Total rolls removed: {}", total);
}
