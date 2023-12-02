use std::{env, fs};

pub fn solve() {
    let file_path: &str = "/Users/ankitraj/aoc2023/aoc2023/src/day1_input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let rows = contents.split("\n");
    let mut val: i128 = 0;
    for row in rows {
        println!("Row: {row}");
        if !row.is_empty() {
            let mut num_as_str:String = "".to_string();
            for chr in row.chars() {
                if chr.is_numeric() {
                    num_as_str.push(chr)
                }
            }
            if num_as_str.len() == 1 {
                num_as_str.push(num_as_str.chars().nth(num_as_str.len()-1).unwrap());
            }
            let mut final_str: String = "".to_string();
            final_str.push(num_as_str.chars().nth(0).unwrap());
            final_str.push(num_as_str.chars().nth(num_as_str.len()-1).unwrap());
            println!("Num value: {final_str}");
            val += final_str.parse::<i128>().unwrap();
        }

    }
    println!("Value : {val}")
}