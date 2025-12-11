use std::fs;

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
struct Range {
    start: i64,
    end: i64
}


fn main() {
    let file_path = "./input.txt";

    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines: Vec<&str> = input.trim().split("\n\n").collect();
    let mut ranges: Vec<Option<Range>> = Vec::new();
    let mut total = 0;
    // Parsing
    for range in lines[0].split("\n") {
        let nums: Vec<&str> = range.split("-").collect();
        ranges.push(Some(Range {
            start: nums[0].parse::<i64>().unwrap(),
            end: nums[1].parse::<i64>().unwrap(),
        }));
    }

    //Throw out any ranges that are contained inside another
    for (i, current_range) in ranges.clone().into_iter().enumerate() {
        match current_range {
            Some(range) => {
                for (j, comp_range) in ranges.clone().into_iter().enumerate() {
                    match comp_range {
                        Some(com_range) => {
                            if com_range.start <= range.start && com_range.end >= range.end && i != j {
                                ranges[i] = None;
                            }
                        },
                        None => {
                            //do nothing
                        }
                    }
                }
            },
            None => {
                //do nothing
            }
        }
    }

    for (i, range) in ranges.clone().into_iter().enumerate() {
        match range {
            Some(mut r) => {
                if  i == 0 {
                    total += (r.end - r.start) + 1;
                    continue;
                }
                // loop through previously calculated id ranges, see if any overlaps with current and
                // calculate new range start or end value
                for j in 0..=i-1 {
                    match ranges[j] {
                        Some(r2) => {
                            // shouldn't need this but the removal above doesn't seem to get all
                            // was good enough for the correct answer though
                            if r.start >= r2.start && r.end <= r2.end {
                                // current range is contained wholely in the prev
                                // set total range to -1 so total calculation evens out
                                r.start = 0;
                                r.end = -1;
                                break;
                            }

                            if r.start > r2.end {
                                // doesn't overlap
                                continue;
                            }else{
                                // overlaps, figure out new start/end
                                if r.start > r2.start {
                                    if r2.end + 1 <= r.end {
                                        // set current range start to prev range end + 1, then continue on to next prev
                                        r.start = r2.end + 1;
                                        continue;
                                    }
                                } else if r.end >= r2.start {
                                    if r2.start - 1 >= r.start {
                                        r.end = r2.start - 1;
                                        continue;
                                    }
                                }
                            }
                        },
                        None => {
                            //do nothing
                        }
                    }
                }
                total += (r.end - r.start) + 1;
            },
            None => {
                //do nothing
            }
        }
    }

    println!("total number of valid ids: {}", total);
}
