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
    let mut unduped_ranges: Vec<Range> = Vec::new();
    // Parsing
    for range in lines[0].split("\n") {
        let nums: Vec<&str> = range.split("-").collect();
        ranges.push(Range {
            start: nums[0].parse::<i64>().unwrap(),
            end: nums[1].parse::<i64>().unwrap(),
        });
    }

    //Throw out any ranges that are contained inside another
    let temp_ranges = ranges.clone();
    for (i, current_range) in temp_ranges.clone().into_iter().enumerate() {
        let mut is_smaller = false;
        for (j, comp_range) in temp_ranges.clone().into_iter().enumerate() {
            if current_range.start >= comp_range.start && current_range.end <= comp_range.end && i != j {
                is_smaller = true;
                break;
            }
        }
        if !is_smaller {
            //isn't smaller than any others, check if it swallows any that have been added
            let mut does_swallow = false;
            for (i, comp_range) in unduped_ranges.clone().into_iter().enumerate() {
                if comp_range.start >= current_range.start && comp_range.end <= current_range.end {
                    unduped_ranges[i] = current_range.clone();
                    does_swallow = true;
                }
            }
            if !does_swallow {
                unduped_ranges.push(current_range.clone());
            }
        }
    }

    for (i, mut range) in unduped_ranges.clone().into_iter().enumerate() {
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
                } else if range.end >= ranges[j].start {
                    if ranges[j].start - 1 >= range.start {
                        range.end = ranges[j].start - 1;
                        continue;
                    }
                }
            }
        }
 
        total += (range.end - range.start) + 1;
    }

    println!("total number of valid ids: {}", total);
}
