use std::fs;

#[derive(Debug)]
struct Step {
    direction: String,
    distance: i16
}

fn main() {
    let file_path = "./src/input.txt";

    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let instructions: Vec<&str> = input.split("\n").collect();
    let mut steps: Vec<Step> = Vec::new();
    let mut pos = 50;
    let mut numHits = 0;

    for instruction in instructions {
        if instruction == "" { continue; }

        let (direction, distance) = instruction.split_at(1);
        steps.push(Step {
            direction: String::from(direction),
            distance: distance.parse::<i16>().unwrap()
        });
    }
    
    for mut step in steps {
        while step.distance > 100 {
            step.distance -= 100;
            numHits += 1;
        }

        if step.direction == "L" {
            pos -= step.distance;
            // wrapped around
            if pos < 0 {
                pos += 100;
                numHits += 1;
            }
        }else{
            pos += step.distance;
            if pos >= 100 {
                pos -= 100;
                numHits += 1;
            }
        }

        if pos == 0 {
            numHits += 1;
        }
    }

    println!("Dial landed on 0 {} times", numHits);
}
