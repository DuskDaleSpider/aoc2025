use std::fs;

#[derive(Debug)]
struct Range {
    start: i64,
    end: i64
}


fn main() {
    let file_path = "./input.txt";

    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines: Vec<&str> = input.trim().split("\n\n").collect();
    let mut ranges: Vec<Range> = Vec::new();
    let mut ids: Vec<i64> = Vec::new();
    let mut total = 0;

    // Parsing
    for range in lines[0].split("\n") {
        let nums: Vec<&str> = range.split("-").collect();
        ranges.push(Range {
            start: nums[0].parse::<i64>().unwrap(),
            end: nums[1].parse::<i64>().unwrap(),
        });
    }

    for id in lines[1].split("\n") {
        let id: i64 = id.parse().unwrap();
        ids.push(id);
    }

    // Searching
    for id in ids {
        for range in &ranges {
            if id >= range.start && id <= range.end {
                total += 1;
                println!("{id} is in range {:?}", range);
                break;
            }
        }
    }

    println!("Number of fresh ids: {total}");
}
