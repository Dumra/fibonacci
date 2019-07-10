use std::io;
use std::cmp::Ordering;

fn main() {
    let mut input_number = String::new();
    io::stdin()
        .read_line(&mut input_number)
        .expect("Can't read from command line");

    let count_fibonacci: u32 = match input_number.trim().parse() {
        Ok(number) => {
            let value: u32 = match (number as u32).cmp(&0) {
                Ordering::Greater => number,
                Ordering::Equal => 1,
                Ordering::Less => 1
            };
            value
        },
        Err(_) => {
            println!("Incorrect number");
            0
        }
    };

    for number in 0..count_fibonacci {
        print!("{}, ", calculate_fibonacci(number));
    }
}

fn calculate_fibonacci(number: u32) -> u32 {
    if number == 0 {
        0
    } else if number == 1 {
        1
    } else {
        calculate_fibonacci(number - 1) + calculate_fibonacci(number - 2)
    }
}
