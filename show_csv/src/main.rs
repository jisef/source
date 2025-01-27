use std::fs;
use std::path::PathBuf;
use std::str::{Lines};
use colored::Colorize;
use pad::{PadStr, Alignment};
use clap::{Arg, Command};

fn main() {

    let matches = Command::new("Show CSV")
        .version("1.0")
        .author("jisef of GH")
        .about("Shows a CSV file")
        .arg(
            Arg::new("filename")
                .help("The file to process")
                .required(true)
                .index(1),
        ).arg(
            Arg::new("seperator")
                .short('s')
                .long("seperator")
                .help("Specifies seperating value")
                .num_args(1)
                .value_parser(clap::builder::ValueParser::new(|s: &str| {
                    if s.len() == 1 {
                        Ok(s.chars().next().unwrap()) // Accept only single-character input
                    } else {
                        Err(format!("Expected a single character, got '{}'", s))
                    }
                }))
                .default_value(","),
        )
        .get_matches();

    let path: &String = matches
        .get_one::<String>("filename")
        .expect("Filename is required");

    let seperator: char = *matches
        .get_one::<char>("seperator")
        .expect("Default value is + ,");



    

    let sigma = read_csv(String::from(path));
    let max_width_per_column = get_max_width_per_column(&sigma, seperator);
    let sigma = sigma.lines();
    print_lines(max_width_per_column, sigma, seperator, 2);
}

fn print_lines(max_width_per_column: Vec<usize>, lines: Lines, seperator: char, offset: usize) {
    let mut ges_width: usize = max_width_per_column.iter().sum();
    ges_width += (offset + 1) * max_width_per_column.len();
    let mut is_first = true;
    let row_seperator = "-".repeat(ges_width);
    
    for line in lines {
        let mut index: usize = 0;
        for item in line.split(seperator) {
            let size:usize = max_width_per_column[index] + offset;

            let mut padded_text = item.pad_to_width_with_alignment(size, Alignment::Middle);
            if is_first {
                padded_text = padded_text + '|'.to_string().as_str();
                print!("{}", padded_text.bold());
            } else {
                padded_text = padded_text + '|'.to_string().as_str();
                print!("{}", padded_text);
            }
            index += 1;
        }
        is_first = false;
        println!();
        println!("{}", row_seperator);
        

    }
}

fn get_max_width_per_column(text: &String, seperator: char) -> Vec<usize>{
    let lines = text.lines();
    let mut list:Vec<usize> = Vec::new();

    let mut is_first: bool = true;
    for line in lines {
        let line = line.split(seperator);
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


fn read_csv(file_path: String) -> String {
    let contents = fs::read_to_string(file_path).unwrap_or_else(|error| {
        println!("Es ist ein Fehler aufgetreten!!! {}", error);
        String::new()
    });
    contents
    //let contents = fs::read_to_string(file_path)
}


// TODO: rework
fn print_help() {
    let width: usize = 10;
    let mut builder = String::new();
    builder.push_str(&*"command <path> <options> \n \n");
    builder.push_str(&*"-h --help :".pad_to_width(width));
    builder.push_str(&*" shows help");

    println!("{}", builder);
}

