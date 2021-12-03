use std::fs;

fn challenge1() {
    let contents = fs::read_to_string("instructions.txt").expect("Failed to load");
    let mut x = 0;
    let mut z = 0;
    for instruction in contents.lines() {
        let bits = instruction.split_once(" ");
        match bits {
            Some((direction, size_str)) => {
                let size: i32 = size_str.parse().expect("failed to parse size");
                match direction {
                    "forward" => x += size,
                    "backward" => x -= size,
                    "up" => z -= size,
                    "down" => z += size,
                    _ => panic!("unknown direction {}", direction)
                }
            }
            None => println!("never mind")
        }
    }
    println!("horizontal position: {}, depth: {}", x, z);
    println!("answer: {}", x * z);
}

fn challenge2() {
    let contents = fs::read_to_string("instructions.txt").expect("Failed to load");
    let mut x = 0;
    let mut z = 0;
    let mut aim = 0;
    for instruction in contents.lines() {
        let bits = instruction.split_once(" ");
        match bits {
            Some((direction, size_str)) => {
                let size: i32 = size_str.parse().expect("failed to parse size");
                match direction {
                    "forward" => { x += size; z += aim * size; }
                    "up" => aim -= size,
                    "down" => aim += size,
                    _ => panic!("unknown direction {}", direction)
                }
            }
            None => println!("never mind")
        }
    }
    println!("horizontal position: {}, depth: {}", x, z);
    println!("answer: {}", x * z);
}

fn main() {
    challenge1();
    challenge2();
}
