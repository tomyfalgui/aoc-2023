use std::fs::read_to_string;

const RED_CUBES: usize = 12;
const GREEN_CUBES: usize = 13;
const BLUE_CUBES: usize = 14;

fn main() {
    let lines = read_lines("input.txt");


    let mut sum = 0;

    'game: for (game, line) in lines.iter().enumerate() {
        let a = line.split(':').collect::<Vec<&str>>();
        let b = a[1].split(';').collect::<Vec<&str>>().iter().map(|game| game.trim_start().split(", ").collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>();

        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;
        println!("{:?}", b);

        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for set in b {
            for cube in set {
                let a = cube.split(' ').collect::<Vec<&str>>();
                let num = a[0].parse::<usize>().unwrap();
                let color = a[1];
                if color == "red" {
                    red += num;
                } else if color == "green" {
                    green += num;
                } else if color == "blue" {
                    blue += num;
                }

                // part 1
                // if red > RED_CUBES || green > GREEN_CUBES || blue > BLUE_CUBES {
                //     continue 'game;
                // }
                if max_red < red {
                    max_red = red;
                }
                if max_green < green {
                    max_green = green;
                }
                if max_blue < blue {
                    max_blue = blue;
                }

            }
            red = 0;
            green = 0;
            blue = 0;


        }

        let product = max_red * max_green * max_blue;
        println!("{}", product);
        sum += product;

    }

    println!("{}", sum);
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .expect("Failed to read file!")
        .lines()
        .map(String::from)
        .collect()
}