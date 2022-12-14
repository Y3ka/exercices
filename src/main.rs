use std::io;

fn main() {
    let mut buf = String::new();
    print!("Enter a list of numbers separated by spaces (e.g 1 4 56 16)");
    io::stdin()
        .read_line(&mut buf)
        .expect("Failed to read line");
    let buf = buf.split(" ");
    let mut integer_list = buf.collect::<Vec<isize>>();
    let mut integer_list = vec![10, 5, 15, 2, 24, 18, 9, 1];
    let median = median(&mut integer_list);
    print!("The sorted array is {:?}\n", integer_list);
    print!("The median is {}", median);
}

fn median(integer_list: &mut Vec<isize>) -> isize {
    integer_list.sort();
    let size: usize = integer_list.len();
    integer_list[(size-1)/2]
}
