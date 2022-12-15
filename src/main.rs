use std::io;
use std::collections::HashMap;

fn main() {
    loop {
        print!("Enter a list of numbers separated by spaces to get the median of the list (e.g 1 4 56 16):\n");
        let mut buf = String::new();
        io::stdin()
            .read_line(&mut buf)
            .expect("Failed to read line");
        let buf = buf.split(" ");
        let str_list = buf.collect::<Vec<&str>>();
        if str_list.len() != 0 {
            let mut integer_list = convert_vec_str_to_integer(str_list);
            let median = median(&mut integer_list);
            let mode = mode(&integer_list);
            print!("The sorted list is {:?}\n", integer_list);
            print!("The median is {}\n", median);
            print!("The value that occurs most often is {}\n", mode);
            break;
        }
        print!("The list cannot be empty!\n");
        
    }
    
}

fn median(integer_list: &mut Vec<isize>) -> f32 {
    integer_list.sort();
    let size: usize = integer_list.len();
    if size % 2 == 0 {
        let value1: f32 = integer_list[(size/2)-1] as f32;
        let value2: f32 = integer_list[size/2] as f32;
        (value1 + value2) / 2.0
    } else {
        integer_list[(size-1)/2] as f32
    }
    
}

fn convert_vec_str_to_integer(str_list: Vec<&str>) -> Vec<isize> {
    let mut integer_list: Vec<isize> = Vec::new();
    for element in &str_list {
        integer_list.push(element.trim().parse::<isize>().expect("Failed converting &str to integer"));
    }
    integer_list
}

fn mode(integer_list: &Vec<isize>) -> &isize {
    let mut occurence = HashMap::new();
    let mut max_count = 0;
    let mut most_present_value = &integer_list[0];
    for element in integer_list {
        let count = occurence.entry(element).or_insert(0);
        *count += 1;
        if *count > max_count {
            max_count = *count;
            most_present_value = element;
        }
    }
    most_present_value
}