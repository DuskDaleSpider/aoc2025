use std::fs;

#[derive(Debug)]
struct Step {
    direction: String,
    distance: i16
}

fn main() {
    let file_path = "./input.txt";

    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let instructions: Vec<&str> = input.split("\n").collect();
    let mut steps: Vec<Step> = Vec::new();
    let mut pos = 50;
    let mut num_hits = 0;

    for instruction in instructions {
        if instruction == "" { continue; }

        let (direction, distance) = instruction.split_at(1);
        steps.push(Step {
            direction: String::from(direction),
            distance: distance.parse::<i16>().unwrap()
        });
    }

    for mut step in steps {
        //process any full rotations
        num_hits += step.distance / 100;
        step.distance = step.distance % 100;

        if step.distance == 0 { continue; }

        //per click calculation solution
        //while step.distance > 0 {
        //    if step.direction == "L" {
        //        pos -= 1;
        //    }else{
        //        pos += 1;
        //    }
        // 
        //    if pos == 0 {
        //        num_hits += 1;
        //    } else if pos == 100 {
        //        pos = 0;
        //        num_hits += 1;
        //    } else if pos == -1 {
        //        pos = 99;
        //    }
        // 
        //    step.distance -= 1;
       // }
        

        //math solution (looking for 5657)
        let started_at_zero = pos == 0;
        if step.direction == "L" {
            pos -= step.distance;
        } else {
            pos += step.distance;
        }

        if pos == 0 {
            num_hits += 1;
        } else if pos < 0 {
            // if the step started at pos 0, click is already counted;
            if !started_at_zero {
                num_hits += 1;
            }
            pos += 100;
        } else if pos >= 100 {
            num_hits += 1;
            pos -= 100;
        }
    }

    println!("Dial passed or landed on 0 {} times", num_hits);
}
