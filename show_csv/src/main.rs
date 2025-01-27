use std::fs;
use std;
use std::str::{Lines, Split};
use colored::Colorize;
use pad::{PadStr, Alignment};

fn main() {
    let seperator: char = ';';
    let sigma = readCSV(String::from("kids.csv"));
    let max_width_per_column = get_max_width_per_column(&sigma, seperator);
    let sigma = sigma.lines();
    print_lines(max_width_per_column, sigma, seperator, 2);
}

fn print_lines(max_width_per_column: Vec<usize>, lines: Lines, seperator: char, spazi: usize) {
    let mut is_first = true;
    for line in lines {
        let mut index: usize = 0;
        for item in line.split(seperator) {
            let size:usize = max_width_per_column[index] + spazi;
            
            let mut padded_text = item.pad_to_width_with_alignment(size, Alignment::Middle);
            if is_first {
                padded_text = padded_text + '|'.to_string().as_str();
                print!("{}", padded_text.bold());
            } else {
                padded_text = padded_text + '|'.to_string().as_str();
                print!("{}", padded_text);
            }
            index += 1;
            is_first = false;
        }
        println!("");
        
    }
}

fn get_max_width_per_column(text: &String, seperator: char) -> Vec<usize>{
    let mut lines = text.lines();
    let mut list:Vec<usize> = Vec::new();

    let mut is_first: bool = true;
    for line in lines {
        let mut line = line.split(seperator);
        let mut index: usize = 0;
        for x in line {
            if is_first {
                list.push(x.len())
            } else {
                let len = x.len();
                if list[index] < len {
                    list[index] = len;
                }
            }
            index = index + 1;
        }
        is_first = false;

    }
    list
}


fn readCSV(file_path: String) -> String {
    let contents = fs::read_to_string(file_path).unwrap_or_else(|error| {
        println!("Es ist ein Fehler aufgetreten!!! {}", error);
        String::new()
    });
    contents
    //let contents = fs::read_to_string(file_path)
}