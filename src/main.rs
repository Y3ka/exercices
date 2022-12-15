use std::io;

fn main() {
    print!("Enter a list of numbers separated by spaces to get the median of the list (e.g 1 4 56 16):\n");
    let mut buf = String::new();
    io::stdin()
        .read_line(&mut buf)
        .expect("Failed to read line");
    let buf = buf.split(" ");
    let str_list = buf.collect::<Vec<&str>>();
    let mut integer_list = convert_vec_str_to_integer(str_list);
    let median = median(&mut integer_list);
    
    print!("The sorted list is {:?}\n", integer_list);
    print!("The median is {}", median);
}

fn median(integer_list: &mut Vec<isize>) -> isize {
    integer_list.sort();
    let size: usize = integer_list.len();
    if size % 2 == 0 {
        let value1: isize = integer_list[(size/2)-1];
        let value2: isize = integer_list[size/2];
        (value1 + value2) / 2
    } else {
        integer_list[(size-1)/2]
    }
    
}

fn convert_vec_str_to_integer(str_list: Vec<&str>) -> Vec<isize> {
    let mut integer_list: Vec<isize> = Vec::new();
    for element in &str_list {
        integer_list.push(element.trim().parse::<isize>().unwrap())
    }
    integer_list
}
