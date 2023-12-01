use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let file_content = read_input("./input.txt");

    let sum: u32 = file_content.split_whitespace()
        .map(|word|
            word.chars().filter_map(|c| c.to_digit(10)).collect::<Vec<u32>>())
        .fold(0, |acc, char| {
            let mut digit = String::new();
            let char_len = char.len();
            let first = char[0];

            match char_len {
                1 => {
                    digit.push_str(first.to_string().as_str());
                    digit.push_str(first.to_string().as_str());
                }
                _ => {
                    let last = char.last().unwrap();
                    digit.push_str(first.to_string().as_str());
                    digit.push_str(last.to_string().as_str());
                }
            }


            let digit: u32 = digit.parse().unwrap();
            println!("{digit}");

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
