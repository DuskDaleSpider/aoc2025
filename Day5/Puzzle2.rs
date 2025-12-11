use std::fs;

#[derive(Debug)]
#[derive(Clone)]
struct Range {
    start: i64,
    end: i64
}


fn main() {
    let file_path = "./input.txt";

    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines: Vec<&str> = input.trim().split("\n\n").collect();
    let mut ranges: Vec<Range> = Vec::new();
    let mut total = 0;
    // Parsing
    for range in lines[0].split("\n") {
        let nums: Vec<&str> = range.split("-").collect();
        ranges.push(Range {
            start: nums[0].parse::<i64>().unwrap(),
            end: nums[1].parse::<i64>().unwrap(),
        });
    }

    for (i, mut range) in ranges.clone().into_iter().enumerate() {
        let mut does_swallow = false;
        let mut largest_swallow = Range {
            start: 0,
            end: 0
        }; 

        if  i == 0 {
            total += (range.end - range.start) + 1;
            continue;
        }
        // loop through previously calculated id ranges, see if any overlaps with current and
        // calculate new range start or end value
        for j in 0..=i-1 {
            if range.start >= ranges[j].start && range.end <= ranges[j].end {
                // current range is contained wholely in the prev
                // set total range to -1 so total caclution evens out
                // also reset does_swallow if it's true
                does_swallow = false;
                range.start = 0;
                range.end = -1;
                break;
            }

            if range.start > ranges[j].end {
                // doesn't overlap
                continue;
            }else{
                // overlaps, figure out new start/end
                if range.start > ranges[j].start {
                    if ranges[j].end + 1 <= range.end {
                        // set current range start to prev range end + 1, then move to next prev
                        range.start = ranges[j].end + 1;
                        continue;
                    }
                } else if range.end >= ranges[j].end {
                    // Current range swallows a prev one 
                    // check agaisnt prev largest swallow if applicable
                    if !does_swallow {
                        does_swallow = true;
                        largest_swallow = ranges[j].clone();
                    } else {
                        if ranges[j].start <= largest_swallow.start && ranges[j].end >= largest_swallow.end {
                            largest_swallow = ranges[j].clone();
                        }
                    }
                    continue;
                } else if range.end >= ranges[j].start {
                    if ranges[j].start - 1 >= range.start {
                        range.end = ranges[j].start - 1;
                        continue;
                    }
                }
            }
        }
 
        //if current did swallow a prev, find the differece
        if does_swallow {
            total += (largest_swallow.start - range.start) + (range.end - largest_swallow.end);
        } else {
            total += (range.end - range.start) + 1;
        }
    }

    println!("total number of valid ids: {}", total);
}
