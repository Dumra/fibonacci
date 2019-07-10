use chrono::Local;
use std::cmp::Ordering;
use std::io;

fn main() {
    let mut input_number = String::new();
    println!("Input ur number:");
    io::stdin()
        .read_line(&mut input_number)
        .expect("Can't read from command line");

    let count_fibonacci: u64 = match input_number.trim().parse() {
        Ok(number) => {
            let value = match (number as u32).cmp(&0) {
                Ordering::Greater => number,
                Ordering::Equal | Ordering::Less => 1,
            };
            value
        }
        Err(_) => {
            println!("Incorrect number");
            0
        }
    };

    let start = Local::now().timestamp();

    let last_loop = count_fibonacci - 1;
    for number in 0..count_fibonacci {
        if number == last_loop {
            print!("{}", calculate_fibonacci(number));
        } else {
            print!("{}, ", calculate_fibonacci(number));
        }
    }
    print!("\n");

    let end = Local::now().timestamp();
    println!("{} seconds for whatever you did.", end - start);
}

fn calculate_fibonacci(number: u64) -> u64 {
    match number {
        0 => 0,
        1 => 1,
        number => calculate_fibonacci(number - 1) + calculate_fibonacci(number - 2),
    }
}
