use std::fs;
use std::path::Path;
use std::collections::HashMap;

fn main() {
    let mut digits_map: HashMap<&str, u32> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9)
    ]);
    let digit_keys = digits_map.keys().map(|k| k.to_string()).collect::<Vec<_>>();

    let file_content = read_input("./input.txt").expect("Erorr reading input file");

    let sum: u32 = file_content.split_whitespace()
        .map(|word|
            {
                let mut digits: Vec<u32> = Vec::new();
                let mut chars: Vec<char> = Vec::new();
                for (i, c) in word.chars().enumerate() {
                    if let Some(digit) = c.to_digit(10) {
                        digits.push(digit);
                    } else {
                        chars.push(c);
                        let key = chars.clone().iter().collect::<String>();
                        for digit_key in &digit_keys {
                            if key.contains(digit_key) {
                                let last_ch = key.clone().chars().last().unwrap();
                                let digit:u32 = digits_map.get(&digit_key as &str).unwrap().clone();
                                digits.push(digit);
                                chars.clear();
                                if last_ch == 'e' || last_ch == 'o' || last_ch == 'n' || last_ch == 't' {
                                    chars.push(last_ch);
                                }
                            }
                        }
                    }
                }


                digits
            }
        )
        .map(|digits| match digits.split_first() {
            Some((first, rest)) => {
                let last = rest.last().unwrap_or(first);
                format!("{}{}", first, last).parse::<u32>().unwrap_or(0)
            }
            None => 0
        }).sum();
    println!("Sum: {}", sum);
}

fn part_1() {
    let file_content = read_input("./input.txt").expect("Erorr reading input file");

    let sum: u32 = file_content.split_whitespace()
        .map(|word|
            word.chars().filter_map(|c| c.to_digit(10)).collect::<Vec<_>>()
        )
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
