use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let file_content = read_input("./input.txt");

    let sum: u32 = file_content.split_whitespace()
        .map(|word|
            word.chars().filter_map(|c| c.to_digit(10)).collect::<Vec<u32>>())
        .fold(0, |acc, digits| {
            let mut digit_string = String::new();
            let digits_len = digits.len();
            let first_digit = digits[0].to_string();

            match digits_len {
                1 => {
                    digit_string = first_digit.repeat(2);
                }
                _ => {
                    let last_digit = digits.last().unwrap().to_string();
                    digit_string = first_digit + &last_digit;
                }
            }


            let digit: u32 = digit_string.parse().unwrap();

            acc + digit
        });

    println!("{}", sum);
}

fn read_input(path: &str) -> String {
    let path = Path::new(path);
    let display = path.display();
    let mut file = match File::open(&path) {
        Err(why) => panic!("Couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => {}
    };

    s
}
