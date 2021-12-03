use std::env;
use std::fs;

fn main() {
    let filename = "input.txt";
    let contents = fs::read_to_string(filename)
        .expect("Could not read file");

    //// Q1 ////
    let mut line_1: i32 = 0;
    let mut larger_count: i32 = -1; // to correct for the first one
    for line in contents.lines() {
        let mut line_0 = line.parse::<i32>().unwrap();
        if line_0 > line_1 {
            larger_count += 1;
        }
        line_1 = line_0;
    }
    println!("The number of increases is: {}", larger_count);

    //// Q2 ////
    let mut line_3: [i32; 3] = [0; 3];
    let mut skip: i32 = 3;
    let mut larger_count: i32 = 0;
    for line in contents.lines() {
        let mut line_0 = line.parse::<i32>().unwrap();
        if skip > 0 {
            skip -= 1;
        } else if line_0 > line_3[2] {
            larger_count += 1;
        }
        line_3 = add_item(line_3, line_0);
    }
    println!("The number of sliding window increases is: {}", larger_count);
}

fn add_item(mut line_3: [i32; 3], insert: i32) -> [i32; 3] {
    line_3[2] = line_3[1];
    line_3[1] = line_3[0];
    line_3[0] = insert;
    return line_3;
}
