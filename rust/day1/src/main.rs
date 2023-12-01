use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let file_content = read_input("./input.txt").expect("Erorr reading input file");

    let sum: u32 = file_content.split_whitespace()
        .map(|word|
            word.chars().filter_map(|c| c.to_digit(10)).collect::<Vec<_>>())
        .map(|digits| match digits.split_first() {
            Some((first, rest)) => {
                let last = rest.last().unwrap_or(first);
                format!("{}{}", first, last).parse::<u32>().unwrap_or(0)
            }
            None => 0
        }).sum();
    println!("Sum: {}", sum);
}

fn read_input(path: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(Path::new(path))
}
