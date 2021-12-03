use std::fs;
use std::str;

fn main() {
    //// Q1 ////
    let contents = fs::read_to_string("input.txt")
        .expect("Could not read file.");

    let mut horizontal: i32 = 0;
    let mut vertical: i32 = 0;

    for line in contents.lines() {
        let mut split: Vec<&str> = line.split_whitespace().collect();
        let mut value = split[1].parse::<i32>().unwrap();
        match split[0] {
            "forward" => horizontal += value,
            "up" => vertical -= value,
            "down" => vertical += value,
            _ => println!("An error occured."),
        }
    }

    let mut result = horizontal * vertical;
    println!("The result of ex.1 is: {}", result);

    //// Q2 ////
    let mut horizontal: i32 = 0;
    let mut depth: i32 = 0;
    let mut aim: i32 = 0;

    for line in contents.lines() {
        let mut split: Vec<&str> = line.split_whitespace().collect();
        let mut value = split[1].parse::<i32>().unwrap();
        match split[0] {
            "forward" =>{horizontal += value; depth += aim * value},
            "up" => aim -= value,
            "down" => aim += value,
            _ => println!("An error occured."),
        }
    }

    let mut result = horizontal * depth;
    println!("The result of ex. 2 is: {}", result);

    
}
