use std::fs;
use std::collections::BTreeMap;

fn get_input() -> String {
    let file_path: &str = "/Users/ankitraj/aoc2023/aoc2023/src/day1_input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    return contents;
}

pub fn solve_part1() {

    let contents = get_input();
    let rows = contents.split("\n");
    let mut val: i128 = 0;
    for row in rows {
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
            val += final_str.parse::<i128>().unwrap();
        }

    }
    println!("Value : {val}")
}

pub fn solve_part2() {
    let contents = get_input();
    let mut res: i128 = 0;
    let rows = contents.split("\n");
    let mut str_num_to_i8_val: BTreeMap<&str, i8> = BTreeMap::new();

    str_num_to_i8_val.insert("one", 1);
    str_num_to_i8_val.insert("two", 2);
    str_num_to_i8_val.insert("three", 3);
    str_num_to_i8_val.insert("four", 4);
    str_num_to_i8_val.insert("five", 5);
    str_num_to_i8_val.insert("six", 6);
    str_num_to_i8_val.insert("seven", 7);
    str_num_to_i8_val.insert("eight", 8);
    str_num_to_i8_val.insert("nine", 9);
    for row in rows {
        if !row.is_empty() {
            let mut num_indexes: BTreeMap<i16, i8> = BTreeMap::new();
            for num in str_num_to_i8_val.keys() {
                let indexes: Vec<_> = row.match_indices(num).collect();
                if !indexes.is_empty(){
                    for item in indexes {
                        num_indexes.insert(item.0 as i16, *str_num_to_i8_val.get(num).unwrap());
                    }

                }
            }
            for (index, chr) in row.chars().enumerate() {
                if chr.is_numeric() {
                    num_indexes.insert(index as i16, chr.to_string().parse::<i8>().unwrap());
                }
            }
            let mut calibrated_values: i128 = 0;
            if num_indexes.len() == 1 {
                let val = *num_indexes.values().nth(0).unwrap();
                calibrated_values = (val*10 + val) as i128
            } else {
                let first_val = *num_indexes.values().nth(0).unwrap() as i128;
                let second_val = *num_indexes.values().nth(num_indexes.len() - 1).unwrap() as i128;
                calibrated_values = first_val*10 + second_val;
            }
            res += calibrated_values;
        }
    }
    println!("Final res: {res}");
}