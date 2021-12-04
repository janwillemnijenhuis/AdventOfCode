use std::fs;

use util::readlines;

fn main() {
    //// Q1 ////
    let contents = fs::read_to_string("input.txt")
        .expect("Could not read from file.");
    let mut bitlen: usize = 12;
    let mut ones_count: Vec<i32> = vec![0;bitlen];
    let mut zeros_count: Vec<i32> = vec![0;bitlen];
    let mut place = 0;
    let mut gamma: Vec<i32> = vec![0;bitlen];
    let mut epsilon: Vec<i32> = vec![0;bitlen];

    // for (i, line) in contents.lines().enumerate() {
    //
    // }

    for line in contents.lines() {
        for c in line.chars() {
            let mut char_i = c as i32;
            match char_i {
                48 => zeros_count[place] += 1,
                49 => ones_count[place] += 1,
                _ => println!("An error occured"),
            }
            place += 1;
        }
        place = 0;
    }

    for i in 0..bitlen {
        if ones_count[i] > zeros_count[i] {
            gamma[i] = 1;
            epsilon[i] = 0;
        } else if zeros_count[i] > ones_count[i] {
            gamma[i] = 0;
            epsilon[i] = 1;
        } else {
            println!("They are equal!");
        }
    }
    let mut gamma2 = gamma.clone();
    let mut epsilon2 = epsilon.clone();
    let mut power_consumption = bitvec_to_int(gamma) * bitvec_to_int(epsilon);
    println!("Result of ex. 1 is: {}", power_consumption);

    //// Q2 ////
    let mut temp_lines: Vec<&str> = Vec::new();
    let mut temp_lines2 = temp_lines.clone();
    for line in contents.lines() {
        if (line.chars().nth(0).unwrap() as i32) == 48 + gamma2[0] {
            temp_lines.push(line);
        }
        if (line.chars().nth(0).unwrap() as i32) == 48 + epsilon2[0] {
            temp_lines2.push(line);
        }
    }

    let mut veclen = bitlen - 1;
    for i in 0..veclen {
        let mut pos = i + 1;
        if temp_lines.len() > 1 {
            temp_lines = update_temp_lines(temp_lines, pos, Some("max"));
        }
        if temp_lines2.len() > 1 {
            temp_lines2 = update_temp_lines(temp_lines2, pos, Some("min"));
        }
    }
    // println!("{:?}", temp_lines);
    // println!("{:?}", temp_lines2);

    let mut oxygen_rating = bitvec_to_int(str_to_bits(temp_lines[0]));
    let mut co2_scrubber = bitvec_to_int(str_to_bits(temp_lines2[0]));

    println!("The result of ex.2 is: {}", oxygen_rating * co2_scrubber);
}

fn bitvec_to_int(vec: Vec<i32>) -> i32 {
    let mut veclen = vec.len();
    let mut result: i32 = 0;
    for i in 0..veclen {
        if vec[i] == 1 {
            let mut power: u32 = (veclen as u32) - (i as u32) - 1;
            let base: i32 = 2;
            result += base.pow(power);
        }
    }
    return result;
}

fn str_to_bits(vec: &str) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    for char in vec.chars() {
        if char as i32 == 49 {
            result.push(1);
        } else if char as i32 == 48 {
            result.push(0);
        } else {
            println!("Error!");
        }
    }
    return result;
}

fn count_max(vec: Vec<&str>, pos: usize, return_type: Option<&str>) -> i32 {
    let mut impl_return_type = return_type.unwrap_or("max");
    let mut one_count = 0;
    let mut zero_count = 0;
    for item in vec {
        if (item.chars().nth(pos).unwrap() as i32) == 49 {
            one_count += 1;
        } else {
            zero_count += 1;
        }
    }
    if impl_return_type == "max" {
        if zero_count > one_count {
            return 0;
        }
    } else {
        if zero_count <= one_count {
            return 0;
        }
    }
    return 1;
}

fn update_temp_lines<'a>(vec: Vec<&'a str>, pos: usize, return_type: Option<&str>) -> Vec<&'a str> {
    let mut impl_return_type = return_type.unwrap_or("max");
    let mut temp_lines: Vec<&str> = Vec::new();
    let mut value = count_max(vec.clone(), pos, Some(impl_return_type));
    for item in vec {
        if (item.chars().nth(pos).unwrap() as i32) == 48 + value {
            temp_lines.push(item);
        }
    }
    return temp_lines;
}